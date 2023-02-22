use num::Float;

use crate::montecarlo::MonteCarlo;

use super::mc_particle::MCParticle;

/// Enum representing the outcome of the current segment.
#[derive(Debug)]
pub enum MCSegmentOutcome {
    Initialize = -1,
    Collision,
    FacetCrossing,
    Census,
}

/// Enum representing the action to take after a particle collides.
#[derive(Debug)]
pub enum MCCollisionEventReturn {
    StopTracking = 0,
    ContinueTracking,
    ContinueCollision,
}

/// Computes the outcome of the current segment for a given particle.
pub fn outcome<T: Float>(
    mcco: &MonteCarlo<T>,
    mc_particle: &MCParticle<T>,
    flux_tally_idx: &usize,
) -> MCSegmentOutcome {
    todo!()
}
