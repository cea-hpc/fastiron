use crate::{
    constants::CustomFloat,
    data::{mc_vector::MCVector, tallies::MCTallyEvent},
    geometry::mc_location::MCLocation,
};

use super::mc_particle::MCParticle;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Species {
    Unknown = -1,
    #[default]
    Known = 0, // \o/
}

/// Structure used to represent a base particle, i.e. a fresh
/// particle with no direction.
#[derive(Debug, Clone, Default)]
pub struct MCBaseParticle<T: CustomFloat> {
    /// Current position
    pub coordinate: MCVector<T>,
    /// Current velocity
    pub velocity: MCVector<T>,
    /// Kinetic energy
    pub kinetic_energy: T,
    /// Weight
    pub weight: T,
    /// Time remaining before this particle hit census
    pub time_to_census: T,
    /// Age
    pub age: T,
    /// Number of mean free paths to a collision
    pub num_mean_free_paths: T,
    /// Number of segments the particle travelled
    pub num_segments: T,

    /// Random number seed for the rng for this particle
    pub random_number_seed: u64,
    /// Unique ID used to identify and track individual particles
    pub identifier: u64,

    /// Last event this particle underwent
    pub last_event: MCTallyEvent,
    /// Species of the particle
    pub species: Species,
    /// Current domain in the spatial grid
    pub domain: usize,
    /// Current cell in the current domain
    pub cell: usize,
}

impl<T: CustomFloat> MCBaseParticle<T> {
    /// Constructor from a [MCParticle] object. To construct from a
    /// [MCBaseParticle] object, we derive the [Clone] trait.
    pub fn new(particle: &MCParticle<T>) -> Self {
        MCBaseParticle {
            coordinate: particle.coordinate,
            velocity: particle.velocity,
            kinetic_energy: particle.kinetic_energy,
            weight: particle.weight,
            time_to_census: particle.time_to_census,
            age: particle.age,
            num_mean_free_paths: particle.num_mean_free_paths,
            num_segments: particle.num_segments,
            random_number_seed: particle.random_number_seed,
            identifier: particle.identifier,
            last_event: particle.last_event,
            species: particle.species,
            domain: particle.domain,
            cell: particle.cell,
        }
    }

    /// Return the current particle's location.
    pub fn get_location(&self) -> MCLocation {
        MCLocation {
            domain: Some(self.domain),
            cell: Some(self.cell),
            facet: Some(0),
        }
    }
}
