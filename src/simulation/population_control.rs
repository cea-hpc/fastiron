//! Code regulating the number of particles in the simulation
//!
//! This module contains the three main functions used to regulate the number
//! of particles in the simulation as well as two internal functions used by
//! those.

use num::{one, zero, FromPrimitive};

use crate::{
    constants::{
        physical::{LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY},
        CustomFloat,
    },
    data::tallies::Balance,
    montecarlo::MonteCarlo,
    particles::{
        mc_base_particle::MCBaseParticle, mc_particle::MCParticle,
        particle_container::ParticleContainer, particle_vault_container::ParticleVaultContainer,
    },
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
    mcco: &mut MonteCarlo<T>,
    container: &mut ParticleContainer<T>,
) {
    let mut target_n_particles: usize = mcco.params.simulation_params.n_particles as usize;
    let mut global_n_particles: usize = 0;
    let local_n_particles: usize = container.processing_particles.len();
    let load_balance = mcco.params.simulation_params.load_balance;

    if load_balance {
        // Spread the target number of particle among the processors
        let tmp: T = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
            / FromPrimitive::from_usize(mcco.processor_info.num_processors).unwrap();
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
            local_n_particles,
            container,
            &mut mcco.particle_vault_container,
            &mut mcco.tallies.balance_task[0],
        );
    }

    mcco.particle_vault_container.collapse_processing();
}

fn population_control_guts<T: CustomFloat>(
    split_rr_factor: T,
    current_n_particles: usize,
    container: &mut ParticleContainer<T>,
    vault_container: &mut ParticleVaultContainer<T>,
    task_balance: &mut Balance,
) {
    //let vault_size = vault_container.vault_size;
    //let mut fill_vault_idx = current_n_particles / vault_size;

    //let mut count: usize = 0;

    if split_rr_factor < one() {
        // too many particles; roll for a kill
        container.processing_particles.retain_mut(|pp| {
            let rand_f: T = rng_sample(&mut pp.random_number_seed);
            if rand_f > split_rr_factor {
                // particle dies
                task_balance.rr += 1;
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
                task_balance.split += 1;
                split_pp.random_number_seed = spawn_rn_seed::<T>(&mut pp.random_number_seed);
                split_pp.identifier = split_pp.random_number_seed;

                container.extra_particles.push(split_pp);
            })
        });
        container.clean_extra_vaults();
    }
    /*
    // march backwards through particles; might be unecessary since we use vectors?
    (0..current_n_particles).rev().for_each(|particle_idx| {
        let vault_idx = particle_idx / vault_size;
        let task_particle_idx = particle_idx % vault_size;

        // since we cant pass around a mutable reference to the inside of an option,
        // we clone the particle and overwrite it.
        if let Some(mut pp) =
            vault_container.get_task_processing_vault(vault_idx)[task_particle_idx].clone()
        {
            count += 1; // count only valid particles
            let rand_f: T = rng_sample(&mut pp.random_number_seed);

            if split_rr_factor < one() {
                // too many particles; roll for a kill
                let task_processing_vault = vault_container.get_task_processing_vault(vault_idx);
                if rand_f > split_rr_factor {
                    task_processing_vault.erase_swap_particles(task_particle_idx);
                    task_balance.rr += 1;
                } else {
                    // update particle & overwrite old version
                    pp.weight /= split_rr_factor;
                    task_processing_vault[task_particle_idx] = Some(pp);
                }
            } else if split_rr_factor > one() {
                // not enough particles; create new ones by splitting
                let mut split_factor = split_rr_factor.floor();
                if rand_f > split_rr_factor - split_factor {
                    split_factor -= one();
                }
                pp.weight /= split_rr_factor;

                // create child particle & add them to vault
                let n_split: usize = split_factor.to_usize().unwrap();
                (0..n_split).for_each(|_| {
                    let mut split_pp = pp.clone();
                    task_balance.split += 1;
                    split_pp.random_number_seed = spawn_rn_seed::<T>(&mut pp.random_number_seed);
                    split_pp.identifier = split_pp.random_number_seed;
                    // add to the vault
                    vault_container.add_processing_particle(split_pp, &mut fill_vault_idx);
                });

                // update original by overwriting it
                vault_container.get_task_processing_vault(vault_idx)[task_particle_idx] = Some(pp);
            }
        }
    });
    */
    // did we really march through all particles?
    //assert_eq!(count, current_n_particles);
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
    vault_container: &mut ParticleVaultContainer<T>,
    task_balance: &mut Balance,
) {
    if low_weight_cutoff > zero() {
        //let current_n_particles = vault_container.particles_processing_size();
        //let vault_size = vault_container.vault_size;

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
                    task_balance.rr += 1;
                    false
                }
            } else {
                // particle survives
                true
            }
        });
        // march backwards through particles; might be unecessary since we use vectors?
        /*
        (0..current_n_particles).rev().for_each(|particle_idx| {
            let vault_idx = particle_idx / vault_size;
            let task_particle_idx = particle_idx % vault_size;

            let task_processing_vault = vault_container.get_task_processing_vault(vault_idx);
            if let Some(mut pp) = task_processing_vault[task_particle_idx].clone() {
                if pp.weight <= weight_cutoff {
                    let rand_f: T = rng_sample(&mut pp.random_number_seed);
                    if rand_f <= low_weight_cutoff {
                        // particle continues with an increased weight
                        pp.weight /= low_weight_cutoff;
                        task_processing_vault[task_particle_idx] = Some(pp);
                    } else {
                        // particle is killed
                        task_processing_vault.erase_swap_particles(task_particle_idx);
                        task_balance.rr += 1;
                    }
                }
            }
        });
        */
        vault_container.collapse_processing();
    }
}

/// Simulates the sources according to the problem's parameters.
///
/// This function spawns particle is source regions. Each time this function
/// is called (once per cycle), 10% of the target number of particles are
/// spawned. _Where_ they are spawned depends on both deterministic factors and
/// randomness.
pub fn source_now<T: CustomFloat>(mcco: &mut MonteCarlo<T>, container: &mut ParticleContainer<T>) {
    let time_step = mcco.time_info.time_step;

    let mut source_rate: Vec<T> = vec![zero(); mcco.material_database.mat.len()];
    (0..mcco.material_database.mat.len()).for_each(|mat_idx| {
        let name = &mcco.material_database.mat[mat_idx].name;
        let sr = mcco.params.material_params[name].source_rate;
        source_rate[mat_idx] = sr;
    });

    let mut total_weight_particles: T = zero();
    mcco.domain.iter().for_each(|dom| {
        dom.cell_state.iter().for_each(|cell| {
            let cell_weight_particles: T = cell.volume * source_rate[cell.material] * time_step;
            total_weight_particles += cell_weight_particles;
        });
    });

    let n_particles = mcco.params.simulation_params.n_particles as usize;

    let source_fraction: T = FromPrimitive::from_f64(0.1).unwrap();

    let source_particle_weight: T = total_weight_particles
        / (source_fraction * FromPrimitive::from_usize(n_particles).unwrap());
    assert_ne!(source_particle_weight, zero());

    mcco.source_particle_weight = source_particle_weight;

    // let vault_size = mcco.particle_vault_container.vault_size;
    //let mut processing_idx = 0; // mcco.particle_vault_container.particles_processing_size() / vault_size;

    // on each domain
    mcco.domain
        .iter_mut()
        .enumerate()
        .for_each(|(domain_idx, dom)| {
            // update the tally separately and merge data after
            // this allows for a read-only iterator
            let mut cell_source_tally: Vec<usize> = vec![0; dom.cell_state.len()];

            // on each cell
            dom.cell_state
                .iter()
                .enumerate()
                .for_each(|(cell_idx, cell)| {
                    let cell_weight_particle: T =
                        cell.volume * source_rate[cell.material] * time_step;

                    // create cell_n_particles and add them to the vaults
                    let cell_n_particles: usize = (cell_weight_particle / source_particle_weight)
                        .floor()
                        .to_usize()
                        .unwrap();
                    cell_source_tally[cell_idx] = cell.source_tally;
                    (0..cell_n_particles).for_each(|_ii| {
                        let mut particle: MCParticle<T> = MCParticle::default();

                        // atomic in original code
                        let mut rand_n_seed = cell_source_tally[cell_idx] as u64;
                        cell_source_tally[cell_idx] += 1;

                        rand_n_seed += cell.id as u64;

                        particle.random_number_seed = spawn_rn_seed::<T>(&mut rand_n_seed);
                        particle.identifier = rand_n_seed;

                        particle.coordinate = generate_coordinate_3dg(
                            &mut particle.random_number_seed,
                            dom,
                            cell_idx,
                        );

                        particle
                            .direction_cosine
                            .sample_isotropic(&mut particle.random_number_seed);

                        // sample energy uniformly in [emin; emax] MeV
                        let range = mcco.params.simulation_params.e_max
                            - mcco.params.simulation_params.e_min;
                        let sample: T = rng_sample(&mut particle.random_number_seed);
                        particle.kinetic_energy =
                            sample * range + mcco.params.simulation_params.e_min;

                        let speed: T = speed_from_energy(particle.kinetic_energy);
                        particle.velocity.x = speed * particle.direction_cosine.alpha;
                        particle.velocity.y = speed * particle.direction_cosine.beta;
                        particle.velocity.z = speed * particle.direction_cosine.gamma;

                        particle.domain = domain_idx;
                        particle.cell = cell_idx;
                        particle.task = 0; // used task_idx in original code but it stayed const
                        particle.weight = source_particle_weight;

                        let mut rand_f: T = rng_sample(&mut particle.random_number_seed);
                        particle.num_mean_free_paths = -one::<T>() * rand_f.ln();
                        rand_f = rng_sample(&mut particle.random_number_seed);
                        particle.time_to_census = time_step * rand_f;

                        let base_particle: MCBaseParticle<T> = MCBaseParticle::new(&particle);
                        container.processing_particles.push(base_particle);
                        //mcco.particle_vault_container
                        //    .add_processing_particle(base_particle, &mut processing_idx);

                        // atomic in original code
                        mcco.tallies.balance_task[particle.task].source += 1;
                    });
                });
            // update source_tally
            (0..dom.cell_state.len()).for_each(|cell_idx| {
                dom.cell_state[cell_idx].source_tally = cell_source_tally[cell_idx];
            });
        });
}

fn speed_from_energy<T: CustomFloat>(energy: T) -> T {
    let rest_mass_energy: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
    let speed_of_light: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();
    speed_of_light
        * (energy * (energy + two * (rest_mass_energy))
            / ((energy + rest_mass_energy) * (energy + rest_mass_energy)))
            .sqrt()
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_speed_from_e() {
        let energy = 15.032;
        let speed = speed_from_energy(energy);
        assert_eq!(speed, 5299286790.50638);
    }
}
