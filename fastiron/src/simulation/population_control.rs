//! Code regulating the number of particles in the simulation
//!
//! This module contains the three main functions used to regulate the number
//! of particles in the simulation as well as two internal functions used by
//! those.

use num::{one, FromPrimitive};

use crate::{
    constants::CustomFloat,
    data::tallies::TalliedEvent,
    montecarlo::{MonteCarloData, MonteCarloUnit},
    particles::{mc_particle::MCParticle, particle_container::ParticleContainer},
    simulation::mct::generate_coordinate_3dg,
    utils::mc_rng_state::{rng_sample, spawn_rn_seed},
};

/// Routine used to compute a split factor, value used to regulate population
/// of the problem.
///
/// If load balancing is enabled, the spawned particle will be spread
/// throughout the threads to reach an even distribution. Else, the spawning/killing
/// of particles will be uniform across threads, ignoring the local population.
/// This is reflected in the split factor computation.
pub fn compute_split_factor<T: CustomFloat>(
    global_target_n_particles: usize,
    global_n_particles: usize,
    local_n_particles: usize,
    num_threads: usize,
    load_balance: bool,
) -> T {
    if load_balance {
        // unit-specific modifications to reach uniform distribution
        if local_n_particles != 0 {
            let local_target_n_particles: usize = {
                let tmp: T = <T as FromPrimitive>::from_usize(global_target_n_particles).unwrap()
                    / FromPrimitive::from_usize(num_threads).unwrap();
                tmp.ceil().to_usize().unwrap()
            };
            // return
            <T as FromPrimitive>::from_usize(local_target_n_particles).unwrap()
                / FromPrimitive::from_usize(local_n_particles).unwrap()
        } else {
            // return
            one()
        }
    } else {
        // uniform modifications across units without regards to distribution
        // return
        <T as FromPrimitive>::from_usize(global_target_n_particles).unwrap()
            / FromPrimitive::from_usize(global_n_particles).unwrap()
    }
}

/// Simulates the sources according to the problem's parameters.
///
/// This function spawns particle is source regions. Each time this function
/// is called (once per cycle), 10% of the target number of particles are
/// spawned. _Where_ they are spawned depends on both deterministic factors and
/// randomness.
///
/// Each cell is given a weight, computed according to source rate of its
/// material, volume and time step: The weight is directly proportional to
/// the number of particle that should be spawned in the cell, during a time
/// interval equal to the time step. Given this weight, and the weight of a
/// fresh particle at sourcing, we can compute the number of particle the cell
/// should spawn to get the overall desired number.
///
/// The amount of particle spawned is a consequence of the source particle weight
/// computation as the value is adjusted according to the problem's total weight.
pub fn source_now<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    let time_step = mcdata.params.simulation_params.dt;

    let source_rate: Vec<T> = mcdata
        .material_database
        .mat
        .iter()
        .map(|mat| mcdata.params.material_params[&mat.name].source_rate)
        .collect();
    let mut seeds: Vec<u64> = Vec::new();

    // on each domain
    mcunit
        .domain
        .cell_state
        .iter_mut()
        .enumerate()
        .for_each(|(cell_idx, cell)| {
            // compute number of particles to be created in the cell
            let cell_weight: T = cell.volume * source_rate[cell.material] * time_step;
            let cell_n_particles: usize = (cell_weight / mcdata.source_particle_weight)
                .floor()
                .to_usize()
                .unwrap();
            seeds.clear();

            for _ in 0..cell_n_particles {
                // source_tally is fetched & incr atomically in original code
                let rand_n_seed = (cell.source_tally + cell.id) as u64;
                cell.source_tally += 1;
                seeds.push(rand_n_seed);
            }

            mcunit.tallies.balance_cycle[TalliedEvent::Source] += seeds.len() as u64;

            container
                .processing_particles
                .extend(seeds.iter_mut().map(|rand_n_seed| {
                    // ~~~ init particle

                    let mut particle: MCParticle<T> = MCParticle::default();

                    particle.random_number_seed = spawn_rn_seed::<T>(rand_n_seed);
                    particle.identifier = *rand_n_seed;
                    particle.coordinate = generate_coordinate_3dg(
                        &mut particle.random_number_seed,
                        &mcunit.domain.mesh,
                        cell_idx,
                        cell.volume,
                    );
                    particle.domain = mcunit.domain.global_domain;
                    particle.cell = cell_idx;
                    particle.weight = mcdata.source_particle_weight;

                    // ~~~ random sampling

                    particle.sample_isotropic();
                    // sample energy uniformly in [emin; emax] MeV
                    let range = mcdata.params.simulation_params.e_max
                        - mcdata.params.simulation_params.e_min;
                    particle.kinetic_energy = rng_sample::<T>(&mut particle.random_number_seed)
                        * range
                        + mcdata.params.simulation_params.e_min;
                    particle.sample_num_mfp();
                    particle.time_to_census =
                        time_step * rng_sample::<T>(&mut particle.random_number_seed);

                    particle
                }));
        });
}
