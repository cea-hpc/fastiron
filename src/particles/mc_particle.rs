//! Extended code for particles
//!
//! This module contains code of an extended particle structure used
//! for computations.

use num::FromPrimitive;

use crate::{
    constants::{
        physical::{LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY},
        CustomFloat,
    },
    data::{direction_cosine::DirectionCosine, mc_vector::MCVector, tallies::MCTallyEvent},
};

/// Custom enum used to model a particle's species.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Species {
    /// Invalid value.
    Unknown = -1,
    #[default]
    /// Valid value. Quicksilver only supportedone particle type.
    Known = 0, // \o/
}

/// Structure used to hold all data of a particle.
///
/// This is mostly used for computations during the tracking section.
#[derive(Debug, Default, Clone)]
pub struct MCParticle<T: CustomFloat> {
    /// Current position.
    pub coordinate: MCVector<T>,
    /// Kinetic energy.
    pub kinetic_energy: T,
    /// Weight.
    pub weight: T,
    /// Time remaining before this particle hit census.
    pub time_to_census: T,
    /// Age.
    pub age: T,
    /// Number of mean free paths to a collision.
    pub num_mean_free_paths: T,
    /// Number of segments the particle travelled.
    pub num_segments: T,

    /// Random number seed for the rng for this particle.
    pub random_number_seed: u64,
    /// Unique ID used to identify and track individual particles.
    pub identifier: u64,

    /// Last event this particle underwent.
    pub last_event: MCTallyEvent,
    /// Species of the particle.
    pub species: Species,
    /// Current domain in the spatial grid.
    pub domain: usize,
    /// Current cell in the current domain.
    pub cell: usize,
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
    /// Update the particle's field to model its movement along the specified
    /// direction and distance
    pub fn move_particle(&mut self, distance: T) {
        self.coordinate += self.direction_cosine.dir * distance;
    }

    pub fn get_speed(&self) -> T {
        let rest_mass_energy: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
        let speed_of_light: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        speed_of_light
            * (self.kinetic_energy * (self.kinetic_energy + two * (rest_mass_energy))
                / ((self.kinetic_energy + rest_mass_energy)
                    * (self.kinetic_energy + rest_mass_energy)))
                .sqrt()
    }
}
