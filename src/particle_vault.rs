use num::Float;

use crate::mc::mc_base_particle::MCBaseParticle;

/// Struture used to group particle in batches.
#[derive(Debug)]
pub struct ParticleVault<T: Float> {
    particles: Vec<MCBaseParticle<T>>,
}
