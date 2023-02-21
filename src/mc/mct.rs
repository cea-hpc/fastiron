use num::Float;
use serde_yaml::Location;

use crate::{montecarlo::MonteCarlo, direction_cosine::DirectionCosine};

use super::{mc_nearest_facet::MCNearestFacet, mc_particle::MCParticle, mc_vector::MCVector, mc_domain::MCDomain, mc_facet_adjacency::SubfacetAdjacency, mc_location::MCLocation};

#[allow(clippy::too_many_arguments)]
pub fn nearest_face<T: Float>(mc_particle: &MCParticle<T>, location: &Location, coord: &MCVector<T>, direction_cosine: &DirectionCosine<T>, distance_threshold: T, current_best_distance: T, new_segment: bool, mcco: &MonteCarlo<T>) -> MCNearestFacet<T> {
    todo!()
}

pub fn generate_coordinate_3dg<T: Float>(seed: u64, domain_num: usize, cell: usize, mcco: &MonteCarlo<T>) -> MCVector<T> {
    todo!()
}

pub fn cell_position_3dg<T: Float>(domain: &MCDomain<T>, cell_idx: usize) -> MCVector<T> {
    todo!()
}

pub fn adjacent_facet<T: Float>(location: &MCLocation, mc_particle: &MCParticle<T>, mcco: &MonteCarlo<T>) -> SubfacetAdjacency {
    todo!()
}

pub fn reflect_particle<T: Float>(mcco: &MonteCarlo<T>, mc_particle: &MCParticle<T>) {
    todo!()
}