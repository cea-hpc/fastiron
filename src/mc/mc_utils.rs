use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::{montecarlo::MonteCarlo, particle_vault::ParticleVault};

use super::mc_particle::MCParticle;

pub fn load_particle<T: Float>(
    mcco: &MonteCarlo<T>,
    mc_particle: &MCParticle<T>,
    particle_vault: &ParticleVault<T>,
    particle_idx: usize,
) {
    todo!()
}

pub fn source_now<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    todo!()
}
