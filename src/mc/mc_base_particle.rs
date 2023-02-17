use crate::tallies::MCTallyEvent;

use super::mc_vector::MCVector;

/// Structure used to represent a base particle, i.e. a fresh
/// particle with no direction.
#[derive(Debug)]
pub struct MCBaseParticle {
    /// Current position
    pub coordinate: MCVector,
    /// Current velocity
    pub velocity: MCVector,
    /// Kinetic energy
    pub kinetic_energy: f64,
    /// Weight
    pub weight: f64,
    /// Time remaining before this particle hit census
    pub time_to_census: f64,
    /// Age
    pub age: f64,
    /// Number of mean free paths to a collision (should be an integer?)
    pub num_mean_free_paths: f64,
    /// Number of segments the particle travelled?
    pub num_segments: f64,

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
