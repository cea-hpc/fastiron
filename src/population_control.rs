use std::{cell::RefCell, rc::Rc};

use num::{one, Float, FromPrimitive};

use crate::{
    montecarlo::MonteCarlo, particle_vault_container::ParticleVaultContainer, tallies::Balance,
};

/// Routine used to monitor and regulate population level.
pub fn population_control<T: Float + FromPrimitive>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    load_balance: bool,
) {
    let mut target_n_particles: usize = mcco.borrow().params.simulation_params.n_particles as usize;
    let mut global_n_particles: usize = 0;
    let local_n_particles: usize = mcco
        .borrow()
        .particle_vault_container
        .particles_processing_size();

    if load_balance {
        target_n_particles /= mcco.borrow().processor_info.num_processors;
    } else {
        global_n_particles = local_n_particles;
    }

    let balance = &mut mcco.borrow_mut().tallies.balance_task[0];
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
            &mut mcco.borrow_mut().particle_vault_container,
            balance,
        );
    }
}

fn population_control_guts<T: Float + FromPrimitive>(
    split_rr_factor: T,
    current_n_particles: usize,
    vault: &mut ParticleVaultContainer<T>,
    task_balance: &mut Balance,
) {
    todo!()
}

/// Play russian-roulette with low-weight particles relative
/// to the source particle weight.
pub fn roulette_low_weight_particles<T: Float + FromPrimitive>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    todo!()
}
