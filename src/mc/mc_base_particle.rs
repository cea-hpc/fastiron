use crate::tallies::MCTallyEvent;

use super::mc_vector::MCVector;

/// Structure used to represent a base particle, i.e. ...
#[derive(Debug)]
pub struct MCBaseParticle {
    pub coordinate: MCVector,
    pub velocity: MCVector,
    pub kinetic_energy: f64,
    pub weight: f64,
    pub time_to_census: f64,
    pub age: f64,
    pub num_mean_free_paths: f64,
    pub num_segments: f64,

    pub random_number_seed: u64,
    pub identifier: u64, // usize?

    pub last_event: MCTallyEvent,
    pub num_collisions: u32,
    pub breed: u32,
    pub species: u32,
    pub domain: u32,
    pub cell: u32,
    // num_base_ints
    // num_base_floats ?
    // num_base_chars
}
