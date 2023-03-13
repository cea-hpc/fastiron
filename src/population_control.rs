use std::fmt::Debug;

use num::{one, Float, FromPrimitive};

use crate::{
    mc::mc_rng_state::{rng_sample, spawn_rn_seed},
    montecarlo::MonteCarlo,
    particle_vault_container::ParticleVaultContainer,
    tallies::Balance,
};

/// Routine used to monitor and regulate population level.
pub fn population_control<T: Float + FromPrimitive + Debug>(mcco: &mut MonteCarlo<T>, load_balance: bool) {
    let mut target_n_particles: usize = mcco.params.simulation_params.n_particles as usize;
    let mut global_n_particles: usize = 0;
    let local_n_particles: usize = mcco.particle_vault_container.particles_processing_size();

    if load_balance {
        target_n_particles /= mcco.processor_info.num_processors;
    } else {
        global_n_particles = local_n_particles;
    }

    let mut split_rr_factor: T = one();
    if load_balance {
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
            &mut mcco.particle_vault_container,
            &mut mcco.tallies.balance_task[0],
        );
    }
}

fn population_control_guts<T: Float + FromPrimitive + Debug>(
    split_rr_factor: T,
    current_n_particles: usize,
    vault: &mut ParticleVaultContainer<T>,
    task_balance: &mut Balance,
) {
    let vault_size = vault.vault_size;
    let mut fill_vault_idx = current_n_particles / vault_size;

    // march backwards through particles; might be unecessary since we use vectors?
    (0..current_n_particles)
        .into_iter()
        .rev()
        .for_each(|particle_idx| {
            //println!("particle_idx: {particle_idx}");
            //println!("vault_size: {vault_size}");
            let vault_idx = particle_idx / vault_size;
            //let task_processing_vault = vault.get_task_processing_vault(vault_idx);
            let task_particle_idx = particle_idx % vault_size;

            // since we cant pass around a mutable reference to the inside of an option,
            // we clone the particle and overwrite it.
            if let Some(mut pp) =
                vault.get_task_processing_vault(vault_idx)[task_particle_idx].clone()
            {
                let rand_n: T = rng_sample(&mut pp.random_number_seed);
                if split_rr_factor < one() {
                    let task_processing_vault = vault.get_task_processing_vault(vault_idx);
                    if rand_n > split_rr_factor {
                        task_processing_vault.erase_swap_particles(task_particle_idx);
                        task_balance.rr += 1;
                    } else {
                        // update particle & overwrite old version
                        pp.weight = pp.weight / split_rr_factor;
                        task_processing_vault[task_particle_idx] = Some(pp);
                    }
                } else if split_rr_factor > one() {
                    let mut split_factor = split_rr_factor.floor();
                    if rand_n > split_rr_factor - split_factor {
                        split_factor = split_factor - one();
                    }
                    pp.weight = pp.weight / split_rr_factor;

                    // create child particle & add them to vault
                    let n_split: usize = split_factor.to_usize().unwrap();
                    (0..n_split).into_iter().for_each(|_| {
                        let mut split_pp = pp.clone();
                        task_balance.split += 1;
                        split_pp.random_number_seed =
                            spawn_rn_seed::<T>(&mut pp.random_number_seed);
                        split_pp.identifier = split_pp.random_number_seed;
                        // add to the vault
                        vault.add_processing_particle(split_pp, &mut fill_vault_idx);
                    });

                    // add original back to the vault
                    // No intermediate variable for the reference to the task processing vault
                    // because we use a mut borrow in the interator above
                    vault.get_task_processing_vault(vault_idx)[task_particle_idx] = Some(pp);
                }
            }
        });
}

/// Play russian-roulette with low-weight particles relative
/// to the source particle weight.
pub fn roulette_low_weight_particles<T: Float + FromPrimitive + Debug>(
    low_weight_cutoff: f64,
    source_particle_weight: f64,
    vault: &mut ParticleVaultContainer<T>,
    task_balance: &mut Balance,
) {
    if low_weight_cutoff > 0.0 {
        let current_n_particles = vault.particles_processing_size();
        let vault_size = vault.vault_size;

        let l_weight_cutoff: T = FromPrimitive::from_f64(low_weight_cutoff).unwrap();
        let weight_cutoff: T =
            FromPrimitive::from_f64(low_weight_cutoff * source_particle_weight).unwrap();

        // march backwards through particles; might be unecessary since we use vectors?
        (0..current_n_particles)
            .into_iter()
            .rev()
            .for_each(|particle_idx| {
                let vault_idx = particle_idx / vault_size;
                let task_particle_idx = if vault_idx == 0 {
                    particle_idx
                } else {
                    particle_idx % vault_idx
                };

                let task_processing_vault = vault.get_task_processing_vault(vault_idx);
                if let Some(mut pp) = task_processing_vault[task_particle_idx].clone() {
                    if pp.weight < weight_cutoff {
                        let rand_n: T = rng_sample(&mut pp.random_number_seed);
                        if rand_n < l_weight_cutoff {
                            // particle continues with an increased weight
                            pp.weight = pp.weight / l_weight_cutoff;
                            task_processing_vault[task_particle_idx] = Some(pp);
                        } else {
                            // particle is killed
                            task_processing_vault.erase_swap_particles(task_particle_idx);
                            task_balance.rr += 1;
                        }
                    }
                }
            });
        vault.collapse_processing();
    }
}
