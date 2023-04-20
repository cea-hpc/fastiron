//! Extended code for particles
//!
//! This module contains code of an extended particle structure used
//! for computations.

use num::zero;

use crate::{constants::CustomFloat, data::direction_cosine::DirectionCosine};

use super::mc_base_particle::MCBaseParticle;

/// Structure used to hold all data of a particle.
///
/// This is mostly used for computations during the tracking section.
#[derive(Debug, Default, Clone)]
pub struct MCParticle<T: CustomFloat> {
    /// Base data of the particle.
    pub base_particle: MCBaseParticle<T>,
    /// Direction of the particle as a normalized `(x, y, z)` vector.
    pub direction_cosine: DirectionCosine<T>,
    /// Cache-ing the current total cross section/
    pub total_cross_section: T,
    /// Distance to a collision.
    pub mean_free_path: T,
    /// Distance this particle travels in a segment.
    pub segment_path_length: T,
    /// Current energy group the particle belong to.
    pub energy_group: usize,
    /// Nearest facet.
    pub facet: usize,
    /// Normal dot product value kept when crossing a facet.
    pub normal_dot: T,
}

impl<T: CustomFloat> MCParticle<T> {
    /// Constructor from a [MCBaseParticle] object.
    pub fn new(from_particle: &MCBaseParticle<T>) -> Self {
        let speed = from_particle.velocity.length();
        let d_cos = DirectionCosine {
            dir: from_particle.velocity * speed.recip(),
        };

        MCParticle {
            base_particle: from_particle.clone(),
            direction_cosine: d_cos,
            total_cross_section: zero(),
            mean_free_path: zero(),
            segment_path_length: zero(),
            energy_group: 0,
            facet: 0,
            normal_dot: zero(),
        }
    }

    /// Update the particle's field to model its movement along the specified
    /// direction and distance
    pub fn move_particle(&mut self, distance: T) {
        self.base_particle.coordinate += self.direction_cosine.dir * distance;
    }
}
