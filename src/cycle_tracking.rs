use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::{mc::mc_particle::MCParticle, montecarlo::MonteCarlo, particle_vault::ParticleVault};

/// Main steps of the CycleTracking sections
pub fn cycle_tracking_guts<T: Float>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    particle_index: usize,
    processing_vault: &ParticleVault<T>,
    processed_vault: &ParticleVault<T>,
) {
    todo!()
}

/// Computations of the CycleTracking sections
pub fn cycle_tracking_function<T: Float>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    mc_particle: &MCParticle<T>,
    particle_index: usize,
    processing_vault: &ParticleVault<T>,
    processed_vault: &ParticleVault<T>,
) {
    todo!()
}
