use std::fmt::Error;

use num::Float;

use crate::tallies::MCTallyEvent;

use super::{mc_vector::MCVector, mc_particle::MCParticle, mc_location::MCLocation};

/// Structure used to represent a base particle, i.e. a fresh
/// particle with no direction.
#[derive(Debug)]
pub struct MCBaseParticle<T: Float> {
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
    /// Number of mean free paths to a collision (should be an integer?)
    pub num_mean_free_paths: T,
    /// Number of segments the particle travelled?
    pub num_segments: T,

    /// Random number seed for the rng for this particle
    pub random_number_seed: u64,
    /// Unique ID used to identify and track individual particles (should be usize?)
    pub identifier: u64, // usize?

    /// Last event this particle underwent
    pub last_event: MCTallyEvent,
    /// Number of collisions the particle underwent?
    pub num_collisions: u32,
    /// Breed of the particle, i.e. how it was produced (should be usize?)
    pub breed: u32,
    /// Species of the particle (should be usize?)
    pub species: u32,
    /// Current domain in the spatial grid (should be usize?)
    pub domain: u32,
    /// Current cell in the current domain (should be usize?)
    pub cell: u32,
    // num_base_ints
    // num_base_floats ?
    // num_base_chars
}

impl<T: Float> MCBaseParticle<T> {
    /// Constructor from a [MCParticle] object. To construct from a 
    /// [MCBaseParticle] object, [Clone] will be implemented.
    pub fn new(particle: &MCParticle<T>) -> Self {
        todo!()
    }

    /// Undefined in original code?
    pub fn particle_id_number(&self) -> u32 {
        todo!()
    }

    /// Invalidate a Particle; This is done by setting its type as UNKNOWN; 
    /// The function will fail if it is already set as UNKNOWN.
    pub fn invalidate(&mut self) -> Result<(), Error> {
        todo!()
    }

    /// Not implementing this one beforehand, will do when necessary
    pub fn serialize() {
        todo!()
    }

    /// Return the current particle's location.
    pub fn get_location(&self) -> MCLocation {
        todo!()
    }

    // Not implementing this one beforehand, will do when necessary
    //pub fn to_string(&self) -> String {
    //    todo!()
    //}

    pub fn typ(&self) -> u32 {
        todo!()
    }
    pub fn index(&self) -> u32 {
        todo!()
    }
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}

impl<T: Float> Default for MCBaseParticle<T> {
    fn default() -> Self {
        todo!()
    }
}

impl<T: Float> Clone for MCBaseParticle<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}