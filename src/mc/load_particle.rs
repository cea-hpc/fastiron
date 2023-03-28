use num::{zero, FromPrimitive};

use crate::{
    constants::{physical::TINY_FLOAT, CustomFloat},
    particle_vault::ParticleVault,
};

use super::mc_particle::MCParticle;

/// Copies a single particle from the particle-vault data and returns it.
pub fn load_particle<T: CustomFloat>(
    particle_vault: &ParticleVault<T>,
    particle_idx: usize,
    ts: T,
) -> Option<MCParticle<T>> {
    if let Some(mut particle) = particle_vault.get_base_particle(particle_idx) {
        // update time to census
        let tiny_f: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
        if particle.time_to_census <= tiny_f {
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
