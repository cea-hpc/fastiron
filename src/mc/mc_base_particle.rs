use std::fmt::Error;

use num::{zero, Float};

use crate::tallies::MCTallyEvent;

use super::{mc_location::MCLocation, mc_particle::MCParticle, mc_vector::MCVector};

/// Structure used to represent a base particle, i.e. a fresh
/// particle with no direction.
#[derive(Debug, Clone)]
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
    /// Species of the particle (should be usize? changed to enum?)
    pub species: i32,
    /// Current domain in the spatial grid (should be usize?)
    pub domain: u32,
    /// Current cell in the current domain (should be usize?)
    pub cell: u32,
}

impl<T: Float> MCBaseParticle<T> {
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
            num_collisions: particle.num_collisions,
            breed: particle.breed,
            species: particle.species,
            domain: particle.domain,
            cell: particle.cell,
        }
    }

    /// Invalidate a Particle; This is done by setting its type as UNKNOWN;
    /// The function will fail if it is already set as UNKNOWN.
    pub fn invalidate(&mut self) -> Result<(), Error> {
        if self.is_valid() {
            self.species = -1;
            return Ok(());
        }
        Err(Error)
    }

    /// Return the current particle's location.
    pub fn get_location(&self) -> MCLocation {
        MCLocation {
            domain: self.domain,
            cell: self.cell,
            facet: 0,
        }
    }

    // Not implementing this one beforehand, will do when necessary
    // pub fn copy_particle_to_string(&self) -> String {
    //    todo!()
    //}

    /// Returns true if the particle is valid, false otherwise.
    pub fn is_valid(&self) -> bool {
        self.species >= 0
    }
}

impl<T: Float> Default for MCBaseParticle<T> {
    fn default() -> Self {
        MCBaseParticle {
            coordinate: Default::default(),
            velocity: Default::default(),
            kinetic_energy: zero(),
            weight: zero(),
            time_to_census: zero(),
            age: zero(),
            num_mean_free_paths: zero(),
            num_segments: zero(),
            random_number_seed: 0,
            identifier: 0,
            last_event: Default::default(),
            num_collisions: 0,
            breed: 0,
            species: -1,
            domain: 0,
            cell: 0,
        }
    }
}
