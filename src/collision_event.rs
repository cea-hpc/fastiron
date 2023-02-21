use num::Float;

use crate::{mc::mc_particle::MCParticle, montecarlo::MonteCarlo};

pub fn collision_event<T: Float>(
    mcco: &mut MonteCarlo<T>,
    mc_particle: &MCParticle<T>,
    tally_idx: usize,
) -> bool {
    todo!()
}
