use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::{montecarlo::MonteCarlo, particle_vault::ParticleVault};

use super::mc_particle::MCParticle;

/// Copies a single particle from the particle-vault data structure into
/// the active-particle data structure.
pub fn load_particle<T: Float>(
    mcco: &MonteCarlo<T>,
    mc_particle: &MCParticle<T>,
    particle_vault: &ParticleVault<T>,
    particle_idx: usize,
) {
    todo!()
}

/// Simulates the sources according to the problem's parameters.
pub fn source_now<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    todo!()
}
