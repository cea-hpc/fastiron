//! Extended code for particles
//!
//! This module contains code of an extended particle structure used
//! for computations.

use std::fmt::Display;

use num::zero;

use crate::{
    constants::CustomFloat, data::direction_cosine::DirectionCosine,
    geometry::mc_location::MCLocation,
};

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
    /// Task working on
    pub task: usize,
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
            task: 0,
            energy_group: 0,
            facet: 0,
            normal_dot: zero(),
        }
    }

    /// Returns the location of the particle as a [MCLocation] object.
    pub fn get_location(&self) -> MCLocation {
        MCLocation {
            domain: Some(self.base_particle.domain),
            cell: Some(self.base_particle.cell),
            facet: Some(self.facet),
        }
    }

    /// Update the particle's field to model its movement along the specified
    /// direction and distance
    pub fn move_particle(&mut self, distance: T) {
        self.base_particle.coordinate += self.direction_cosine.dir * distance;
    }
}

// replaces original method `PrintParticle`
impl<T: CustomFloat> Display for MCParticle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "coordinate: {} {} {}",
            self.base_particle.coordinate.x,
            self.base_particle.coordinate.y,
            self.base_particle.coordinate.z
        )?;
        writeln!(
            f,
            "velocity: {} {} {}",
            self.base_particle.velocity.x,
            self.base_particle.velocity.y,
            self.base_particle.velocity.z
        )?;
        writeln!(
            f,
            "direction cosine: {} {} {}",
            self.direction_cosine.dir.x, self.direction_cosine.dir.y, self.direction_cosine.dir.z
        )?;
        writeln!(f, "kinetic energy: {}", self.base_particle.kinetic_energy)?;
        writeln!(f, "weight: {}", self.base_particle.weight)?;
        writeln!(f, "time to census: {}", self.base_particle.time_to_census)?;
        writeln!(f, "total cross section: {}", self.total_cross_section)?;
        writeln!(f, "age: {}", self.base_particle.age)?;
        writeln!(
            f,
            "num mean free paths: {}",
            self.base_particle.num_mean_free_paths
        )?;
        writeln!(f, "mean free path: {}", self.mean_free_path)?;
        writeln!(f, "segment path length: {}", self.segment_path_length)?;
        writeln!(
            f,
            "random number seed: {}",
            self.base_particle.random_number_seed
        )?;
        writeln!(f, "identifier: {}", self.base_particle.identifier)?;
        writeln!(f, "last event: {:?}", self.base_particle.last_event)?;
        writeln!(f, "num segments: {}", self.base_particle.num_segments)?;
        writeln!(f, "task: {}", self.task)?;
        writeln!(f, "species: {:?}", self.base_particle.species)?;
        writeln!(f, "energy group: {}", self.energy_group)?;
        writeln!(f, "domain: {}", self.base_particle.domain)?;
        writeln!(f, "cell: {}", self.base_particle.cell)?;
        writeln!(f, "facet: {}", self.facet)?;
        writeln!(f, "normal dot: {}", self.normal_dot)
    }
}

impl<T: CustomFloat> PartialEq for MCParticle<T> {
    fn eq(&self, other: &Self) -> bool {
        // is this enough?
        self.base_particle.identifier == other.base_particle.identifier
    }
}
