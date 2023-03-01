use core::panic;

use num::{Float, FromPrimitive, zero};

use crate::{direction_cosine::{DirectionCosine}, montecarlo::MonteCarlo, physical_constants::HUGE_FLOAT};

use super::{
    mc_distance_to_facet::MCDistanceToFacet, mc_domain::MCDomain,
    mc_facet_adjacency::SubfacetAdjacency, mc_location::MCLocation,
    mc_nearest_facet::MCNearestFacet, mc_particle::MCParticle, mc_vector::MCVector,
};

/// Computes which facet of the specified cell is nearest 
/// to the specified coordinates.
#[allow(clippy::too_many_arguments)]
pub fn nearest_facet<T: Float + FromPrimitive>(
    mc_particle: &mut MCParticle<T>,
    distance_threshold: T,
    current_best_distance: T,
    new_segment: bool,
    mcco: &MonteCarlo<T>,
) -> MCNearestFacet<T> {
    // check if location is somewhat invalid
    //if (location.cell < 0) | (location.cell < 0) {
    //    panic!()
    //}
    let location = mc_particle.get_location();
    let domain = &mcco.domain[location.domain];

    let mut nearest_facet = mct_nf_3dg(mc_particle, domain);
    
    if nearest_facet.distance_to_facet < zero() {
        nearest_facet.distance_to_facet = zero();
    }

    if nearest_facet.distance_to_facet > FromPrimitive::from_f64(HUGE_FLOAT).unwrap() {
        panic!()
    }

    nearest_facet
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

// ==============================
//       Private functions
// ==============================

fn mct_nf_3dg<T: Float + FromPrimitive>(
    particle: &mut MCParticle<T>,
    domain: &MCDomain<T>,
) -> MCNearestFacet<T> {
    let location = particle.get_location();
    let coords = particle.coordinate;
    let direction_cosine = particle.direction_cosine.clone();
    todo!()
}

fn mct_cell_volume_3dg_vector_tetdet<T: Float>(
    v0: &MCVector<T>,
    v1: &MCVector<T>,
    v2: &MCVector<T>,
    v3: &MCVector<T>,
) -> T {
    todo!()
}

fn mct_nf_3dg_move_particle<T: Float>(
    domain: &MCDomain<T>,
    location: &MCLocation,
    coord: &mut MCVector<T>,
    move_factor: T,
) {
    todo!()
}

fn mct_nf_compute_nearest<T: Float>(
    num_facets_per_cell: usize,
    distance_to_facet: &mut MCDistanceToFacet<T>,
) -> MCNearestFacet<T> {
    todo!()
}

#[allow(clippy::too_many_arguments)]
fn mct_nf_find_nearest<T: Float>(
    mc_particle: &mut MCParticle<T>,
    domain: &MCDomain<T>,
    location: &MCLocation,
    coord: &MCVector<T>,
    iteration: &mut usize,
    move_factor: &mut T,
    num_facets_per_cell: usize,
    distance_to_facet: &mut MCDistanceToFacet<T>,
    retry: &mut bool,
) -> MCNearestFacet<T> {
    todo!()
}

fn mct_facet_points_3dg<T: Float>(
    domain: &MCDomain<T>,
    cell: usize,
    facet: usize,
    num_points_per_facet: usize,
) -> usize {
    todo!()
}

#[allow(clippy::too_many_arguments)]
fn mct_nf_3dg_dist_to_segment<T: Float>(
    plane_tolerance: T,
    facet_normal_dot_dcos: T,
    aa: T,
    bb: T,
    cc: T,
    dd: T,
    facet_coords0: &MCVector<T>,
    facet_coords1: &MCVector<T>,
    facet_coords2: &MCVector<T>,
    coords: &MCVector<T>,
    d_cos: &DirectionCosine<T>,
    allow_enter: bool,
) -> T {
    todo!()
}