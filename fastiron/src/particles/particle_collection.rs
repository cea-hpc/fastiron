//! Data structure used to hold particles
//!
//! This module contains code used as an abstraction of particle vectors. This
//! allows for custom iterator implementation.

use crate::constants::CustomFloat;

use super::mc_particle::MCParticle;

pub struct ParticleCollection<T: CustomFloat> {
    data: Vec<MCParticle<T>>,
}

struct ParParticleIter<'a, T: CustomFloat> {
    data_slice: &'a [MCParticle<T>],
}
