use crate::constants::CustomFloat;

use super::{mc_particle::MCParticle, particle_collection::ParticleCollection};

#[derive(Debug)]
pub struct ParticleGrinder<T: CustomFloat> {
    pub chamber: [MCParticle<T>; 128],
    pub to_process: ParticleCollection<T>,
    pub processed: ParticleCollection<T>,
}

impl<T: CustomFloat> ParticleGrinder<T> {
    pub fn grind(&mut self) {
        todo!()
    }

    pub fn reload(&mut self) -> bool {
        todo!()
    }
}
