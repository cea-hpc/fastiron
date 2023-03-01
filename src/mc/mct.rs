use num::Float;

use crate::{direction_cosine::DirectionCosine, montecarlo::MonteCarlo};

use super::{
    mc_domain::MCDomain, mc_facet_adjacency::SubfacetAdjacency, mc_location::MCLocation,
    mc_nearest_facet::MCNearestFacet, mc_particle::MCParticle, mc_vector::MCVector,
};

/// Computes which facet is the nearest to a given particle.
#[allow(clippy::too_many_arguments)]
pub fn nearest_facet<T: Float>(
    mc_particle: &MCParticle<T>,
    location: &MCLocation,
    coord: &MCVector<T>,
    direction_cosine: &DirectionCosine<T>,
    distance_threshold: T,
    current_best_distance: T,
    new_segment: bool,
    mcco: &MonteCarlo<T>,
) -> MCNearestFacet<T> {
    todo!()
}

/// Generates a random coordinate inside a polyhedral cell.
pub fn generate_coordinate_3dg<T: Float>(
    seed: u64,
    domain_num: usize,
    cell: usize,
    mcco: &MonteCarlo<T>,
) -> MCVector<T> {
    todo!()
}

/// Returns a coordinate that represents the "center" of the cell
pub fn cell_position_3dg<T: Float>(domain: &MCDomain<T>, cell_idx: usize) -> MCVector<T> {
    todo!()
}

/// Returns the adjacency of the given cell.
pub fn adjacent_facet<T: Float>(
    // ORIGINAL FUNCTION IS IN ITS OWN FILE
    location: &MCLocation,
    mc_particle: &MCParticle<T>,
    mcco: &MonteCarlo<T>,
) -> SubfacetAdjacency {
    todo!()
}

/// Reflects a particle off a reflection-type boundary.
pub fn reflect_particle<T: Float>(mcco: &MonteCarlo<T>, mc_particle: &MCParticle<T>) {
    todo!()
}
