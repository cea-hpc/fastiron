use num::Float;

use crate::{montecarlo::MonteCarlo, particle_vault::ParticleVault, tallies::MCTallyEvent};

use super::mc_particle::MCParticle;

/// Computes and transform accordingly a [MCParticle] object crossing a facet.
pub fn event<T: Float>(
    mc_particle: &MCParticle<T>,
    mcco: &MonteCarlo<T>,
    particle_idx: usize,
    processing_vault: &ParticleVault<T>,
) -> MCTallyEvent {
    todo!()
}
