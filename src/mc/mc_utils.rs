use std::{cell::RefCell, rc::Rc};

use num::{zero, Float};

use crate::{montecarlo::MonteCarlo, particle_vault::ParticleVault};

use super::mc_particle::MCParticle;

/// Copies a single particle from the particle-vault data and returns it.
pub fn load_particle<T: Float>(
    mcco: &MonteCarlo<T>,
    particle_vault: &ParticleVault<T>,
    particle_idx: usize,
) -> MCParticle<T> {
    let mut particle = particle_vault.get_particle(particle_idx).unwrap();

    // update time to census
    if particle.time_to_census <= zero() {
        particle.time_to_census = particle.time_to_census + mcco.time_info.time_step;
    }
    // set age
    if particle.age < zero() {
        particle.age = zero();
    }
    // get energy group
    particle.energy_group = mcco.nuclear_data.get_energy_groups(particle.kinetic_energy);

    particle
}

/// Simulates the sources according to the problem's parameters.
pub fn source_now<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    todo!()
}
