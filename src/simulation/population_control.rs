//! Code regulating the number of particles in the simulation
//!
//! This module contains the three main functions used to regulate the number
//! of particles in the simulation as well as two internal functions used by
//! those.

use num::{one, zero, FromPrimitive};

use crate::{
    constants::CustomFloat,
    data::tallies::Balance,
    montecarlo::{MonteCarloData, MonteCarloUnit},
    particles::{mc_particle::MCParticle, particle_container::ParticleContainer},
    simulation::mct::generate_coordinate_3dg,
    utils::mc_rng_state::{rng_sample, spawn_rn_seed},
};

/// Routine used to monitor and regulate population level.
///
/// If load balancing is enabled, the spawned particle will be spread
/// throughout the processors. Using the current number of particle and
/// the target number of particles, the function computes a split factor.
/// If the split factor is strictly below one, there are too many particles,
/// if it is striclty superior to one, there are too little. Particles are
/// then either randomly killed or spawned to get to the desired number.
pub fn population_control<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    let mut target_n_particles: usize = mcdata.params.simulation_params.n_particles as usize;
    let mut global_n_particles: usize = 0;
    let local_n_particles: usize = container.processing_particles.len();
    let load_balance = mcdata.params.simulation_params.load_balance;

    if load_balance {
        // Spread the target number of particle among the processors
        let tmp: T = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
            / FromPrimitive::from_usize(mcdata.exec_info.num_threads).unwrap();
        target_n_particles = tmp.ceil().to_usize().unwrap();
    } else {
        global_n_particles = local_n_particles;
    }

    let mut split_rr_factor: T = one();
    if load_balance {
        // Compute processor-specific split factor
        if local_n_particles != 0 {
            split_rr_factor = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
                / FromPrimitive::from_usize(local_n_particles).unwrap();
        }
    } else {
        split_rr_factor = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
            / FromPrimitive::from_usize(global_n_particles).unwrap();
    }

    if split_rr_factor != one() {
        population_control_guts(
            split_rr_factor,
            container,
            &mut mcunit.tallies.balance_cycle,
        );
    }
}

fn population_control_guts<T: CustomFloat>(
    split_rr_factor: T,
    container: &mut ParticleContainer<T>,
    balance: &mut Balance,
) {
    if split_rr_factor < one() {
        // too many particles; roll for a kill
        container.processing_particles.retain_mut(|pp| {
            let rand_f: T = rng_sample(&mut pp.random_number_seed);
            if rand_f > split_rr_factor {
                // particle dies
                balance.rr += 1;
                false
            } else {
                // particle survives with increased weight
                pp.weight /= split_rr_factor;
                true
            }
        });
    } else if split_rr_factor > one() {
        // not enough particles; create new ones by splitting
        container.processing_particles.iter_mut().for_each(|pp| {
            let rand_f: T = rng_sample(&mut pp.random_number_seed);
            let mut split_factor = split_rr_factor.floor();
            if rand_f > split_rr_factor - split_factor {
                split_factor -= one();
            }
            pp.weight /= split_rr_factor;

            let n_split: usize = split_factor.to_usize().unwrap();
            (0..n_split).for_each(|_| {
                let mut split_pp = pp.clone();
                balance.split += 1;
                split_pp.random_number_seed = spawn_rn_seed::<T>(&mut pp.random_number_seed);
                split_pp.identifier = split_pp.random_number_seed;

                container.extra_particles.push(split_pp);
            })
        });
        container.clean_extra_vaults();
    }
}

/// Play russian-roulette with low-weight particles relative
/// to the source particle weight.
///
/// This function regulates the number of low (statistica) weight particle to
/// prevent clusters of low energy particle from falsifying the results.
pub fn roulette_low_weight_particles<T: CustomFloat>(
    low_weight_cutoff: T,
    source_particle_weight: T,
    container: &mut ParticleContainer<T>,
    balance: &mut Balance,
) {
    if low_weight_cutoff > zero() {
        let weight_cutoff = low_weight_cutoff * source_particle_weight;

        container.processing_particles.retain_mut(|pp| {
            if pp.weight <= weight_cutoff {
                let rand_f: T = rng_sample(&mut pp.random_number_seed);
                if rand_f <= low_weight_cutoff {
                    // particle survives with increased weight
                    pp.weight /= low_weight_cutoff;
                    true
                } else {
                    // particle dies
                    balance.rr += 1;
                    false
                }
            } else {
                // particle survives
                true
            }
        });
    }
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

    let mut total_unit_weight: T = zero();
    mcunit.domain.iter().for_each(|dom| {
        dom.cell_state.iter().for_each(|cell| {
            // constant because cell volume is constant in our program
            let cell_weight: T = cell.volume * source_rate[cell.material] * time_step;
            total_unit_weight += cell_weight;
        });
    });

    let n_particles = mcdata.params.simulation_params.n_particles as usize;

    let source_fraction: T = FromPrimitive::from_f64(0.1).unwrap();

    // single particle weight for the number of particle to be spawned in the unit
    // to be source_fraction*n_particles
    let source_particle_weight: T =
        total_unit_weight / (source_fraction * FromPrimitive::from_usize(n_particles).unwrap());
    assert_ne!(source_particle_weight, zero());

    mcunit.source_particle_weight = source_particle_weight;

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
                    let cell_n_particles: usize = (cell_weight / source_particle_weight)
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
                        particle.weight = source_particle_weight;

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
