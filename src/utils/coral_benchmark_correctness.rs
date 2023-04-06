//! Code used to run additionnal tests when becnhmarking
//!
//! This code is only called if the running examples are specific benchmarks.
//! If there is a need for additional checks on data yielded by a _homemade_
//! example, this is where to start.

use num::{one, zero, FromPrimitive};

use crate::{
    constants::CustomFloat,
    data::tallies::Tallies,
    parameters::{BenchType, Parameters},
};

/// Runs additionnal tests according to the [BenchType].
pub fn coral_benchmark_correctness<T: CustomFloat>(params: &Parameters<T>, tallies: &Tallies<T>) {
    if params.simulation_params.coral_benchmark == BenchType::Standard {
        return;
    }

    // only on mpi rank 0 in QS code
    balance_ratio_test(params, tallies);
    balance_event_test(tallies);
    missing_particle_test(tallies);

    // add a condition on cycles/total particles?
    fluence_test(tallies);
}

/// Test Balance Tallies for relative correctness.
///
/// Expected ratios of absorbs, fissions, scatters are maintained
/// withing some tolerance, based on input expectation.
pub fn balance_ratio_test<T: CustomFloat>(params: &Parameters<T>, tallies: &Tallies<T>) {
    println!("Testing if ratios for absorbtion, fission & scattering are maintained...");

    let balance_tally = &tallies.balance_cumulative;
    let absorb: T = FromPrimitive::from_u64(balance_tally.absorb).unwrap();
    let fission: T = FromPrimitive::from_u64(balance_tally.fission).unwrap();
    let scatter: T = FromPrimitive::from_u64(balance_tally.scatter).unwrap();
    let (fission_ratio, scatter_ratio, absorb_ratio, percent_tolerance): (T, T, T, T) =
        if params.simulation_params.coral_benchmark == BenchType::Coral1 {
            (
                FromPrimitive::from_f64(0.05).unwrap(),
                FromPrimitive::from_f64(1.0).unwrap(),
                FromPrimitive::from_f64(0.04).unwrap(),
                one(),
            )
        } else {
            // then it's BenchType::Coral2
            // can be verified from parsing & first of of the calling function
            (
                FromPrimitive::from_f64(0.075).unwrap(),
                FromPrimitive::from_f64(0.830).unwrap(),
                FromPrimitive::from_f64(0.094).unwrap(),
                FromPrimitive::from_f64(1.1).unwrap(),
            )
        };
    let tolerance = percent_tolerance / FromPrimitive::from_f64(100.0).unwrap();

    let abs2sct: T = ((absorb / absorb_ratio) * (scatter_ratio / scatter) - one()).abs();
    let abs2fsn: T = ((absorb / absorb_ratio) * (fission_ratio / fission) - one()).abs();
    let sct2abs: T = ((scatter / scatter_ratio) * (absorb_ratio / absorb) - one()).abs();
    let sct2fsn: T = ((scatter / scatter_ratio) * (fission_ratio / fission) - one()).abs();
    let fsn2abs: T = ((fission / fission_ratio) * (absorb_ratio / absorb) - one()).abs();
    let fsn2sct: T = ((fission / fission_ratio) * (scatter_ratio / scatter) - one()).abs();

    // pass if no ratio is above tolerance
    let pass: bool = !((abs2sct > tolerance)
        | (abs2fsn > tolerance)
        | (sct2abs > tolerance)
        | (sct2fsn > tolerance)
        | (fsn2abs > tolerance)
        | (fsn2sct > tolerance));
    if pass {
        println!("PASS:: Ratios are maintained within {percent_tolerance}%");
    } else {
        println!("FAIL:: Ratios are not maintained with {percent_tolerance}% tolerance");
        println!("Total absorb: {absorb}");
        println!("Total scatter: {scatter}");
        println!("Total fission: {fission}");
        println!("absorb to scatter: {abs2sct}");
        println!("absorb to fission: {abs2fsn}");
        println!("scatter to absorb: {sct2abs}");
        println!("scatter to fission: {sct2fsn}");
        println!("fission to absorb: {fsn2abs}");
        println!("fission to scatter: {fsn2sct}");
    }
}

/// Test Balance Tallies for equality in number of facet crossing
/// and collision events.
pub fn balance_event_test<T: CustomFloat>(tallies: &Tallies<T>) {
    println!("Testing balance between number of facet crossings and reactions...");

    let balance_tally = &tallies.balance_cumulative;
    let facet_crossing: T = FromPrimitive::from_u64(
        balance_tally.num_segments - balance_tally.collision - balance_tally.census,
    )
    .unwrap();
    let collision: T = FromPrimitive::from_u64(balance_tally.collision).unwrap();

    let ratio: T = (facet_crossing / collision - one()).abs();
    let percent_tolerance: T = one();

    let pass: bool = ratio <= percent_tolerance / (FromPrimitive::from_f64(100.0).unwrap());
    if pass {
        println!("PASS:: Ratio maintained within {percent_tolerance}%");
    } else {
        println!("FAIL:: Ratio not maintained within {percent_tolerance}%");
        println!("facet crossing to collision: {ratio}");
    }
}

/// Test for lost particles during the simulation.
///
/// This test should always succeed unless test for
/// done was broken, or we are running with 1 MPI rank
/// and so never preform this test duing test_for_done
pub fn missing_particle_test<T: CustomFloat>(tallies: &Tallies<T>) {
    println!("Testing for lost / unaccounted for particles in this simulation...");

    let bt = &tallies.balance_cumulative;
    let gains: u64 = bt.start + bt.source + bt.produce + bt.split;
    let losses: u64 = bt.absorb + bt.census + bt.escape + bt.rr + bt.fission;

    if gains == losses {
        println!("PASS:: No particles lost during run");
    } else {
        println!("FAIL:: Particles lost during run");
    }
}

/// Test that the scalar flux is homogenous across cells for the problem.
///
/// This test really requires alot of particles or cycles or both
/// This solution should converge to a homogenous solution
pub fn fluence_test<T: CustomFloat>(tallies: &Tallies<T>) {
    println!("Testing fluence for homogeneity across the cells");
    let mut max_diff: T = zero();
    tallies.fluence.domain.iter().for_each(|dom| {
        let mut local_sum: T = zero();
        dom.cell.iter().for_each(|val| local_sum += *val);

        let average: T = local_sum / FromPrimitive::from_usize(dom.size()).unwrap();
        dom.cell.iter().for_each(|cell_value| {
            let percent_diff: T = (*cell_value - average).abs()
                / ((*cell_value + average) / FromPrimitive::from_f64(2.0).unwrap())
                * FromPrimitive::from_f64(100.0).unwrap();
            max_diff = max_diff.max(percent_diff);
        });
    });
    let percent_tolerance: T = FromPrimitive::from_f64(6.0).unwrap();
    let pass = max_diff <= percent_tolerance;
    if pass {
        println!("PASS:: Fluence is homogeneous across cells within {percent_tolerance}%");
    } else {
        println!("FAIL:: Fluence is not homogeneous across cells within {percent_tolerance}%");
        println!("Current max difference: {max_diff}%");
        println!("Try running more particles / cycles to check if the max difference % goes down");
    }
}
