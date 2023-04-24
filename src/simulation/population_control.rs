//! Code regulating the number of particles in the simulation
//!
//! This module contains the three main functions used to regulate the number
//! of particles in the simulation as well as two internal functions used by
//! those.

use num::{one, FromPrimitive};

use crate::{
    constants::CustomFloat,
    data::tallies::Balance,
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
    let mut split_rr_factor: T = one();

    if load_balance {
        // unit-specific modifications to reach uniform distribution
        if local_n_particles != 0 {
            let local_target_n_particles: usize = {
                let tmp: T = <T as FromPrimitive>::from_usize(global_target_n_particles).unwrap()
                    / FromPrimitive::from_usize(num_threads).unwrap();
                tmp.ceil().to_usize().unwrap()
            };
            split_rr_factor = <T as FromPrimitive>::from_usize(local_target_n_particles).unwrap()
                / FromPrimitive::from_usize(local_n_particles).unwrap();
        }
    } else {
        // uniform modifications across units without regards to distribution
        split_rr_factor = <T as FromPrimitive>::from_usize(global_target_n_particles).unwrap()
            / FromPrimitive::from_usize(global_n_particles).unwrap();
    }

    split_rr_factor
}

/// Routine used to monitor and regulate population level according to the specified
/// split factor.
///
/// If the split factor is strictly below one, there are too many particles,
/// if it is striclty superior to one, there are too little: Particles are
/// either randomly killed or spawned to get to the target number of particle.
pub fn regulate<T: CustomFloat>(
    split_rr_factor: T,
    container: &mut ParticleContainer<T>,
    balance: &mut Balance,
) {
    let old_len = container.processing_particles.len();
    if split_rr_factor < one() {
        // too many particles; roll for a kill
        container
            .processing_particles
            .retain_mut(|pp| pp.over_populated_rr(split_rr_factor));
        // deduce number of killed particle from length diff
        balance.rr += (old_len - container.processing_particles.len()) as u64;
    } else if split_rr_factor > one() {
        // not enough particles; create new ones by splitting
        container.processing_particles.iter_mut().for_each(|pp| {
            container
                .extra_particles
                .extend(pp.under_populated_split(split_rr_factor));
        });
        container.clean_extra_vaults();
        balance.split += (container.processing_particles.len() - old_len) as u64;
    }
}

/// Play russian-roulette with low-weight particles relative
/// to the source particle weight.
///
/// This function regulates the number of low (statistical) weight particle to
/// prevent clusters of low energy particle from falsifying the results.
pub fn roulette_low_weight_particles<T: CustomFloat>(
    relative_weight_cutoff: T,
    source_particle_weight: T,
    container: &mut ParticleContainer<T>,
    balance: &mut Balance,
) {
    let old_len = container.processing_particles.len();
    container
        .processing_particles
        .retain_mut(|pp| pp.low_weight_rr(relative_weight_cutoff, source_particle_weight));
    balance.rr += (old_len - container.processing_particles.len()) as u64;
}

/// Simulates the sources according to the problem's parameters.
///
/// This function spawns particle is source regions. Each time this function
/// is called (once per cycle), 10% of the target number of particles are
/// spawned. _Where_ they are spawned depends on both deterministic factors and
/// randomness.
///
/// TODO: Detail the weight system
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

    // on each domain
    mcunit
        .domain
        .iter_mut()
        .enumerate()
        .for_each(|(domain_idx, dom)| {
            // on each cell
            dom.cell_state
                .iter_mut()
                .enumerate()
                .for_each(|(cell_idx, cell)| {
                    // compute number of particles to be created in the cell
                    let cell_weight: T = cell.volume * source_rate[cell.material] * time_step;
                    let cell_n_particles: usize = (cell_weight / mcdata.source_particle_weight)
                        .floor()
                        .to_usize()
                        .unwrap();

                    let mut seeds: Vec<u64> = Vec::with_capacity(cell_n_particles);
                    for _ in 0..cell_n_particles {
                        // source_tally is fetched & incr atomically in original code
                        let rand_n_seed = (cell.source_tally + cell.id) as u64;
                        cell.source_tally += 1;
                        seeds.push(rand_n_seed);
                    }

                    // create cell_n_particles
                    let sourced = seeds.iter_mut().map(|rand_n_seed| {
                        // ~~~ init particle

                        let mut particle: MCParticle<T> = MCParticle::default();

                        particle.random_number_seed = spawn_rn_seed::<T>(rand_n_seed);
                        particle.identifier = *rand_n_seed;
                        particle.coordinate = generate_coordinate_3dg(
                            &mut particle.random_number_seed,
                            &dom.mesh,
                            cell_idx,
                            cell.volume,
                        );
                        particle.domain = domain_idx;
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
                    });

                    // atomic in original code
                    mcunit.tallies.balance_cycle.source += sourced.len() as u64;

                    container.processing_particles.extend(sourced);
                });
        });
}
