//! Base code for particles
//!
//! This module contains the code of the basic particle structure.

use crate::{
    constants::CustomFloat,
    data::{mc_vector::MCVector, tallies::MCTallyEvent},
};

use super::mc_particle::MCParticle;

/// Custom enum used to model a particle's species.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Species {
    /// Invalid value.
    Unknown = -1,
    #[default]
    /// Valid value. Quicksilver only supportedone particle type.
    Known = 0, // \o/
}

/// Structure used to hold base data of a particle.
///
/// This is mostly used to store particle using a reduced amount of memory.
#[derive(Debug, Clone, Default)]
pub struct MCBaseParticle<T: CustomFloat> {
    /// Current position.
    pub coordinate: MCVector<T>,
    /// Current velocity.
    pub velocity: MCVector<T>,
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
}

impl<T: CustomFloat> MCBaseParticle<T> {
    /// Constructor from a [MCParticle] object. To construct from a
    /// [MCBaseParticle] object, we derive the [Clone] trait.
    pub fn new(particle: &MCParticle<T>) -> Self {
        particle.base_particle.clone()
    }
}
