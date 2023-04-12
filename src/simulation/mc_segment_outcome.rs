//! Code for segment outcome computation, i.e. whichever event the particle
//! undergoes next
//!
//! This module contains the function computing which event a given particle
//! will undergo next. This is done by using the notion of _distance_ to an event
//! and finding the minimum distance between all the possible events.\
//! The particle's position and time to census is also updated, while
//! event-specific modifications are left to event-specific functions. The result
//! is returned using an enum ([`MCSegmentOutcome`])that takes value according to
//! the event.

use core::panic;
use std::fmt::Debug;

use num::{zero, FromPrimitive};

use crate::{
    constants::{
        sim::{HUGE_FLOAT, SMALL_FLOAT, TINY_FLOAT},
        CustomFloat,
    },
    data::tallies::MCTallyEvent,
    geometry::facets::MCNearestFacet,
    montecarlo::MonteCarlo,
    particles::mc_particle::MCParticle,
    simulation::{macro_cross_section::weighted_macroscopic_cross_section, mct::nearest_facet},
    utils::mc_rng_state::rng_sample,
};

/// Enum representing the outcome of the current segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MCSegmentOutcome {
    /// Unused. **May be removed**.
    Initialize = -1,
    /// Value for collision event.
    Collision = 0,
    /// Value for facet crossing event.
    FacetCrossing = 1,
    /// Value for census, i.e. no event.
    Census = 2,
}

/// Computes the outcome of the current segment for a given particle.
///
/// Three outcomes are possible for a given particle: census, facet crossing or
/// collision. The distances are computed as follows:
///
/// - Census: The distance is simply equal to the speed of the particle multiplied
///   by the time left until census.
/// - Facet crossing: The distance is computed in a similar way by the function
///   [`nearest_facet()`].
/// - Collision: The distance is computed using probabilities.
pub fn outcome<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    particle: &mut MCParticle<T>,
    flux_tally_idx: usize,
) -> MCSegmentOutcome {
    // initialize distances and constants
    const N_EVENTS: usize = 3;
    let one: T = FromPrimitive::from_f64(1.0).unwrap();
    let huge_f: T = FromPrimitive::from_f64(HUGE_FLOAT).unwrap();
    let small_f: T = FromPrimitive::from_f64(SMALL_FLOAT).unwrap();
    let tiny_f: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
    let mut distance: [T; N_EVENTS] = [huge_f; N_EVENTS];

    let particle_speed = particle.base_particle.velocity.length();

    let mut force_collision = false;
    if particle
        .base_particle
        .num_mean_free_paths
        .is_sign_negative()
    {
        force_collision = true;
        particle.base_particle.num_mean_free_paths = small_f;
    }

    // randomly determines the distance to the next collision
    // based upon the current cell data
    let macroscopic_total_xsection = weighted_macroscopic_cross_section(
        mcco,
        particle.base_particle.domain,
        particle.base_particle.cell,
        particle.energy_group,
    );

    particle.total_cross_section = macroscopic_total_xsection;
    if macroscopic_total_xsection == zero() {
        particle.mean_free_path = huge_f;
    } else {
        particle.mean_free_path = one / macroscopic_total_xsection;
    }

    // if zero
    if particle.base_particle.num_mean_free_paths == zero() {
        let rdm_number: T = rng_sample(&mut particle.base_particle.random_number_seed);
        particle.base_particle.num_mean_free_paths = -one * rdm_number.ln();
    }

    // sets distance to collision, nearest facet and census

    // collision
    if force_collision {
        distance[MCSegmentOutcome::Collision as usize] = small_f;
    } else {
        distance[MCSegmentOutcome::Collision as usize] =
            particle.base_particle.num_mean_free_paths * particle.mean_free_path;
    }
    // census
    distance[MCSegmentOutcome::Census as usize] =
        particle_speed * particle.base_particle.time_to_census;

    // nearest facet
    let nearest_facet: MCNearestFacet<T> = nearest_facet(particle, mcco);
    particle.normal_dot = nearest_facet.dot_product;
    distance[MCSegmentOutcome::FacetCrossing as usize] = nearest_facet.distance_to_facet;

    // exit if the tracker failed to bound the particle's volume
    if particle.base_particle.last_event == MCTallyEvent::FacetCrossingTrackingError {
        return MCSegmentOutcome::FacetCrossing;
    }
    // force a collision if needed
    if force_collision {
        distance[MCSegmentOutcome::FacetCrossing as usize] = huge_f;
        distance[MCSegmentOutcome::Census as usize] = huge_f;
        distance[MCSegmentOutcome::Collision as usize] = tiny_f;
    }

    // pick the outcome and update the particle

    let segment_outcome = find_min(&distance);

    if distance[segment_outcome as usize].is_sign_negative() {
        panic!()
    }
    particle.segment_path_length = distance[segment_outcome as usize];
    particle.base_particle.num_mean_free_paths -=
        particle.segment_path_length / particle.mean_free_path;

    // update the last event
    particle.base_particle.last_event = match segment_outcome {
        MCSegmentOutcome::Initialize => panic!(),
        MCSegmentOutcome::Collision => MCTallyEvent::Collision,
        MCSegmentOutcome::FacetCrossing => MCTallyEvent::FacetCrossingTransitExit,
        MCSegmentOutcome::Census => MCTallyEvent::Census,
    };

    // set the segment path length according to the minimum computed distance
    match segment_outcome {
        MCSegmentOutcome::Collision => particle.base_particle.num_mean_free_paths = zero(),
        MCSegmentOutcome::FacetCrossing => particle.facet = nearest_facet.facet,
        MCSegmentOutcome::Census => {
            particle.base_particle.time_to_census =
                zero::<T>().min(particle.base_particle.time_to_census)
        }
        MCSegmentOutcome::Initialize => panic!(),
    }

    if force_collision {
        particle.base_particle.num_mean_free_paths = zero();
    }

    // skip tallies & early return if the path length is 0
    if particle.segment_path_length == zero() {
        return segment_outcome;
    }

    // move particle to the end of the segment
    particle.move_particle(particle.segment_path_length);

    // decrement time to census & increment age
    let segment_path_time = particle.segment_path_length / particle_speed;
    particle.base_particle.time_to_census -= segment_path_time;
    particle.base_particle.age += segment_path_time;
    if particle.base_particle.time_to_census < zero() {
        particle.base_particle.time_to_census = zero();
    }

    // update scalar flux tally
    // atomic in original code
    mcco.tallies.scalar_flux_domain[particle.base_particle.domain].task[flux_tally_idx].cell
        [particle.base_particle.cell][particle.energy_group] +=
        particle.segment_path_length * particle.base_particle.weight;

    segment_outcome
}

fn find_min<T: CustomFloat>(distance: &[T]) -> MCSegmentOutcome {
    let mut min_val: T = distance[0];
    let mut min_idx: usize = 0;
    (0..distance.len()).for_each(|idx| {
        if distance[idx] < min_val {
            min_idx = idx;
            min_val = distance[idx];
        }
    });

    match min_idx {
        0 => MCSegmentOutcome::Collision,
        1 => MCSegmentOutcome::FacetCrossing,
        2 => MCSegmentOutcome::Census,
        _ => panic!(),
    }
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::sim::{HUGE_FLOAT, SMALL_FLOAT, TINY_FLOAT};
    use num::zero;

    #[test]
    fn find_min_dist() {
        let mut distance: [f64; 3] = [zero(); 3];
        distance[MCSegmentOutcome::Collision as usize] = HUGE_FLOAT;
        distance[MCSegmentOutcome::FacetCrossing as usize] = SMALL_FLOAT;
        distance[MCSegmentOutcome::Census as usize] = TINY_FLOAT;

        let outcome = find_min(&distance);
        assert_eq!(outcome, MCSegmentOutcome::Census);
    }
}
