use core::panic;

use num::{zero, Float, FromPrimitive};

use super::{
    mc_distance_to_facet::MCDistanceToFacet, mc_domain::MCDomain,
    mc_facet_adjacency::SubfacetAdjacency, mc_location::MCLocation,
    mc_nearest_facet::MCNearestFacet, mc_particle::MCParticle, mc_rng_state::rng_sample,
    mc_vector::MCVector,
};
use crate::{
    direction_cosine::DirectionCosine, montecarlo::MonteCarlo, physical_constants::HUGE_FLOAT,
};

const N_POINTS_PER_FACET: usize = 3;

/// Computes which facet of the specified cell is nearest
/// to the specified coordinates.
#[allow(clippy::too_many_arguments)]
pub fn nearest_facet<T: Float + FromPrimitive>(
    mc_particle: &mut MCParticle<T>,
    //distance_threshold: T,
    //current_best_distance: T,
    //new_segment: bool,
    mcco: &MonteCarlo<T>,
) -> MCNearestFacet<T> {
    // check if location is somewhat invalid; need to find an alternative to their magic value
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
/// May be possible to remove the MonteCarlo argument by directly
/// passing a a reference to the domain since its read only.
pub fn generate_coordinate_3dg<T: Float + FromPrimitive>(
    seed: &mut u64,
    domain_num: usize,
    cell_idx: usize,
    mcco: &MonteCarlo<T>,
) -> MCVector<T> {
    let mut coordinate: MCVector<T> = MCVector::default(); // result
    let six: T = FromPrimitive::from_f64(6.0).unwrap();
    let one: T = FromPrimitive::from_f64(1.0).unwrap();

    let domain: &MCDomain<T> = &mcco.domain[domain_num];

    let num_facets: usize = domain.mesh.cell_connectivity[cell_idx].facet.len();
    if num_facets == 0 {
        return coordinate;
    }

    let center: MCVector<T> = cell_position_3dg(domain, cell_idx);
    let rdm_number: T = rng_sample(seed);
    let which_volume = rdm_number * six * domain.cell_state[cell_idx].volume;

    let mut current_volume: T = zero();
    let mut facet_idx: usize = 0;

    let mut point0: MCVector<T> = Default::default();
    let mut point1: MCVector<T> = Default::default();
    let mut point2: MCVector<T> = Default::default();

    // find the facet to sample from
    while current_volume < which_volume {
        if facet_idx == num_facets {
            break;
        }
        let facet_points = mct_facet_points_3dg(domain, cell_idx, facet_idx);

        point0 = domain.mesh.node[facet_points[0]];
        point1 = domain.mesh.node[facet_points[1]];
        point2 = domain.mesh.node[facet_points[2]];

        let subvolume = mct_cell_volume_3dg_vector_tetdet(&point0, &point1, &point2, &center);
        current_volume = current_volume + subvolume;

        facet_idx += 1;
    }
    // the facet we sample from is facet_idx-1; this is due to a change in the loop structure
    // no need to update facet_idx though, it is not used again

    // sample and adjust
    let mut r1: T = rng_sample(seed);
    let mut r2: T = rng_sample(seed);
    let mut r3: T = rng_sample(seed);
    if r1 + r2 > one {
        r1 = one - r1;
        r2 = one - r2;
    }
    if r2 + r3 > one {
        let tmp = r3;
        r3 = one - r1 - r2;
        r2 = one - tmp;
    } else if r1 + r2 + r3 > one {
        let tmp = r3;
        r3 = r1 + r2 + r3 - one;
        r1 = one - r2 - tmp;
    }
    let r4: T = one - r1 - r2 - r3;

    // TODO: replace using the defined operators of MCVector
    coordinate.x = r4 * center.x + r1 * point0.x + r2 * point1.x + r3 * point2.x;
    coordinate.y = r4 * center.y + r1 * point0.y + r2 * point1.y + r3 * point2.y;
    coordinate.z = r4 * center.z + r1 * point0.z + r2 * point1.z + r3 * point2.z;

    coordinate
}

/// Returns a coordinate that represents the "center" of the cell
pub fn cell_position_3dg<T: Float + FromPrimitive>(domain: &MCDomain<T>, cell_idx: usize) -> MCVector<T> {
    let mut coordinate: MCVector<T> = Default::default();

    let n_points: usize = domain.mesh.cell_connectivity[cell_idx].point.len();

    (0..n_points).into_iter().for_each(|point_idx| {
        let point = domain.mesh.cell_connectivity[cell_idx].point[point_idx];
        coordinate += domain.mesh.node[point];
    });

    coordinate /= FromPrimitive::from_usize(n_points).unwrap();

    coordinate
}

/// ONLY USED FOR READABILITY
/// TODO: REMOVE
pub fn adjacent_facet<T: Float>(
    // ORIGINAL FUNCTION IS IN ITS OWN FILE
    location: &MCLocation,
    mc_particle: &MCParticle<T>,
    mcco: &MonteCarlo<T>,
) -> SubfacetAdjacency {
    /* 
    let domain = &mcco.domain[location.domain];
    let adjacency = domain.mesh.cell_connectivity[location.cell].facet[location.facet].subfacet;
    adjacency
    */
    todo!()
}

/// Reflects a particle off a reflection-type boundary.
pub fn reflect_particle<T: Float + FromPrimitive>(mcco: &MonteCarlo<T>, particle: &mut MCParticle<T>) {
    let mut new_d_cos = particle.direction_cosine.clone();
    let location = particle.get_location();
    // direct access replace get_domain method from MCLocation
    let domain = &mcco.domain[location.domain]; 
    let plane = &domain.mesh.cell_geometry[location.cell][location.facet];

    let facet_normal: MCVector<T> = MCVector { x: plane.a, y: plane.b, z: plane.c };

    let two: T = FromPrimitive::from_f64(2.0).unwrap();
    let dot: T = two * (new_d_cos.alpha*facet_normal.x + new_d_cos.beta*facet_normal.y + new_d_cos.gamma*facet_normal.z);

    if dot > zero() {
        new_d_cos.alpha = new_d_cos.alpha - dot*facet_normal.x;
        new_d_cos.beta = new_d_cos.beta - dot*facet_normal.y;
        new_d_cos.gamma = new_d_cos.gamma - dot*facet_normal.z;
        particle.direction_cosine = new_d_cos;
    }
    let particle_speed = particle.velocity.length();
    particle.velocity.x = particle_speed * particle.direction_cosine.alpha;
    particle.velocity.y = particle_speed * particle.direction_cosine.beta;
    particle.velocity.z = particle_speed * particle.direction_cosine.gamma;

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
) -> [usize; N_POINTS_PER_FACET] {
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
