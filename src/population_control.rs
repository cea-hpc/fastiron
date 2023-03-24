use num::{one, zero, FromPrimitive};

use crate::{
    constants::CustomFloat,
    mc::mc_rng_state::{rng_sample, spawn_rn_seed},
    montecarlo::MonteCarlo,
    particle_vault_container::ParticleVaultContainer,
    tallies::Balance,
};

/// Routine used to monitor and regulate population level.
pub fn population_control<T: CustomFloat>(mcco: &mut MonteCarlo<T>, load_balance: bool) {
    let mut target_n_particles: usize = mcco.params.simulation_params.n_particles as usize;
    let mut global_n_particles: usize = 0;
    let local_n_particles: usize = mcco.particle_vault_container.particles_processing_size();

    if load_balance {
        let tmp: T = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
            / FromPrimitive::from_usize(mcco.processor_info.num_processors).unwrap();
        target_n_particles = tmp.ceil().to_usize().unwrap();
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

    mcco.particle_vault_container.collapse_processing();
}

fn population_control_guts<T: CustomFloat>(
    split_rr_factor: T,
    current_n_particles: usize,
    vault: &mut ParticleVaultContainer<T>,
    task_balance: &mut Balance,
) {
    let vault_size = vault.vault_size;
    let mut fill_vault_idx = current_n_particles / vault_size;

    let mut count: usize = 0;

    // march backwards through particles; might be unecessary since we use vectors?
    (0..current_n_particles).rev().for_each(|particle_idx| {
        let vault_idx = particle_idx / vault_size;
        let task_particle_idx = particle_idx % vault_size;

        // since we cant pass around a mutable reference to the inside of an option,
        // we clone the particle and overwrite it.
        if let Some(mut pp) = vault.get_task_processing_vault(vault_idx)[task_particle_idx].clone()
        {
            count += 1; // count only valid particles
            let rand_n: T = rng_sample(&mut pp.random_number_seed);

            if split_rr_factor < one() {
                // too many particles; roll for a kill
                let task_processing_vault = vault.get_task_processing_vault(vault_idx);
                if rand_n > split_rr_factor {
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
                if rand_n > split_rr_factor - split_factor {
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
                    vault.add_processing_particle(split_pp, &mut fill_vault_idx);
                });

                // update original by overwriting it
                vault.get_task_processing_vault(vault_idx)[task_particle_idx] = Some(pp);
            }
        }
    });
    // did we really march through all particles?
    assert_eq!(count, current_n_particles);
}

/// Play russian-roulette with low-weight particles relative
/// to the source particle weight.
pub fn roulette_low_weight_particles<T: CustomFloat>(
    low_weight_cutoff: T,
    source_particle_weight: T,
    vault: &mut ParticleVaultContainer<T>,
    task_balance: &mut Balance,
) {
    if low_weight_cutoff > zero() {
        let current_n_particles = vault.particles_processing_size();
        let vault_size = vault.vault_size;

        let weight_cutoff = low_weight_cutoff * source_particle_weight;

        // march backwards through particles; might be unecessary since we use vectors?
        (0..current_n_particles).rev().for_each(|particle_idx| {
            let vault_idx = particle_idx / vault_size;
            let task_particle_idx = particle_idx % vault_size;

            let task_processing_vault = vault.get_task_processing_vault(vault_idx);
            if let Some(mut pp) = task_processing_vault[task_particle_idx].clone() {
                if pp.weight <= weight_cutoff {
                    let rand_n: T = rng_sample(&mut pp.random_number_seed);
                    if rand_n <= low_weight_cutoff {
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
        vault.collapse_processing();
    }
}
