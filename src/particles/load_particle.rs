use num::zero;

use crate::constants::CustomFloat;

use super::{mc_particle::MCParticle, particle_vault::ParticleVault};

/// Copies a single particle from the particle-vault data and returns it.
pub fn load_particle<T: CustomFloat>(
    particle_vault: &ParticleVault<T>,
    particle_idx: usize,
    ts: T,
) -> Option<MCParticle<T>> {
    if let Some(mut particle) = particle_vault.get_base_particle(particle_idx) {
        // update time to census
        if particle.time_to_census <= zero() {
            particle.time_to_census += ts;
        }

        // set age
        if particle.age < zero() {
            particle.age = zero();
        }

        return Some(MCParticle::new(&particle));
    }
    None
}
