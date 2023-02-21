use num::Float;

use crate::montecarlo::MonteCarlo;

use super::mc_particle::MCParticle;

#[derive(Debug)]
pub enum MCSegmentOutcome {
    Initialize = -1,
    Collision,
    FacetCrossing,
    Census,
}

#[derive(Debug)]
pub enum MCCollisionEventReturn {
    StopTracking = 0,
    ContinueTracking,
    ContinueCollision,
}

pub fn outcome<T: Float>(mcco: &MonteCarlo<T>, mc_particle: &MCParticle<T>, flux_tally_idx: &usize) -> MCSegmentOutcome {
    todo!()
}