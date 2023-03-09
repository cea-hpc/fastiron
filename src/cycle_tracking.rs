use std::{cell::RefCell, rc::Rc, fmt::Display, ops::AddAssign};

use num::{Float, FromPrimitive, one};

use crate::{mc::{mc_particle::MCParticle, mc_utils::load_particle, mc_segment_outcome::outcome}, montecarlo::MonteCarlo, particle_vault::ParticleVault};

/// Main steps of the CycleTracking sections
pub fn cycle_tracking_guts<T: Float + FromPrimitive + Display + AddAssign>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    particle_idx: usize,
    processing_vault: &mut ParticleVault<T>,
    processed_vault: &mut ParticleVault<T>,
) {
    let mut particle = load_particle(&mcco.borrow(), processing_vault, particle_idx);
    particle.task = 0;
    
    cycle_tracking_function(mcco, &mut particle, particle_idx, processing_vault, processed_vault);

    processing_vault.invalidate_particle(particle_idx);
}

/// Computations of the CycleTracking sections
pub fn cycle_tracking_function<T: Float + FromPrimitive + Display + AddAssign>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    particle: &mut MCParticle<T>,
    particle_idx: usize,
    processing_vault: &mut ParticleVault<T>,
    processed_vault: &mut ParticleVault<T>,
) {
    todo!()
}
