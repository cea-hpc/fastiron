use num::Float;

use crate::{direction_cosine::DirectionCosine, tallies::MCTallyEvent};

use super::mc_vector::MCVector;

/// Structure used to represent a particle.
#[derive(Debug)]
pub struct MCParticle<T: Float> {
    /// Current position
    pub coordinate: MCVector<T>,
    /// Current velocity
    pub velocity: MCVector<T>,
    /// Direction of the particle
    pub direction_cosine: DirectionCosine<T>,
    /// Kinetic energy
    pub kinetic_energy: T,
    /// Weight
    pub weight: T,
    /// Time remaining before this particle hit census
    pub time_to_census: T,
    /// Cache-ing the current total cross section
    pub total_cross_section: T,
    /// Age
    pub age: T,
    /// Number of mean free paths to a collision (should be an integer?)
    pub num_mean_free_paths: T,
    /// Distance to a collision
    pub mean_free_path: T,
    /// Distance this particle travels in a segment
    pub segment_path_length: T,
    /// Random number seed for the rng for this particle
    pub random_number_seed: u64,
    /// Unique ID used to identify and track individual particles (should be usize?)
    pub identifier: u64,
    /// Last event this particle underwent
    pub last_event: MCTallyEvent,
    /// Number of collisions the particle underwent?
    pub num_collisions: u32,
    /// Number of segments the particle travelled?
    pub num_segments: u32,
    /// Task working on (should be usize?)
    pub task: u32,
    /// Species of the particle (should be usize?)
    pub species: u32,
    /// Breed of the particle, i.e. how it was produced (should be usize?)
    pub breed: u32,
    //// Current energy group the particle belong to (should be usize?)
    pub energy_group: u32,
    /// Current domain in the spatial grid (should be usize?)
    pub domain: u32,
    /// Current cell in the current domain (should be usize?)
    pub cell: u32,
    /// Facet to be crossed? (should be usize?)
    pub facet: u32,
    /// When crossing a facet, keep the surface normal dot product
    pub normal_dot: T,
}
