use core::panic;

use num::{zero, Float, FromPrimitive};

use super::{
    mc_distance_to_facet::MCDistanceToFacet, mc_domain::MCDomain, 
    mc_facet_geometry::MCGeneralPlane,
    mc_location::MCLocation, mc_nearest_facet::MCNearestFacet, mc_particle::MCParticle,
    mc_rng_state::rng_sample, mc_vector::MCVector,
};
use crate::{
    direction_cosine::DirectionCosine,
    mc::mc_base_particle::Species,
    montecarlo::MonteCarlo,
    physical_constants::{HUGE_FLOAT, SMALL_FLOAT},
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

    // pass the domain directly as argument instead; will change when the function is called.
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

    coordinate = center*r4 + point0*r1 + point1*r2 + point2*r3;

    coordinate
}

/// Returns a coordinate that represents the "center" of the cell
pub fn cell_position_3dg<T: Float + FromPrimitive>(
    domain: &MCDomain<T>,
    cell_idx: usize,
) -> MCVector<T> {
    let mut coordinate: MCVector<T> = Default::default();

    let n_points: usize = domain.mesh.cell_connectivity[cell_idx].point.len();

    (0..n_points).into_iter().for_each(|point_idx| {
        let point = domain.mesh.cell_connectivity[cell_idx].point[point_idx];
        coordinate += domain.mesh.node[point];
    });

    coordinate /= FromPrimitive::from_usize(n_points).unwrap();

    coordinate
}

/// Reflects a particle off a reflection-type boundary.
pub fn reflect_particle<T: Float + FromPrimitive>(
    mcco: &MonteCarlo<T>,
    particle: &mut MCParticle<T>,
) {
    let mut new_d_cos = particle.direction_cosine.clone();
    let location = particle.get_location();
    
    let domain = &mcco.domain[location.domain];
    let plane = &domain.mesh.cell_geometry[location.cell][location.facet];

    let facet_normal: MCVector<T> = MCVector {
        x: plane.a,
        y: plane.b,
        z: plane.c,
    };

    let two: T = FromPrimitive::from_f64(2.0).unwrap();
    let dot: T = two
        * (new_d_cos.alpha * facet_normal.x
            + new_d_cos.beta * facet_normal.y
            + new_d_cos.gamma * facet_normal.z);

    if dot > zero() {
        new_d_cos.alpha = new_d_cos.alpha - dot * facet_normal.x;
        new_d_cos.beta = new_d_cos.beta - dot * facet_normal.y;
        new_d_cos.gamma = new_d_cos.gamma - dot * facet_normal.z;
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
    let huge_f: T = FromPrimitive::from_f64(HUGE_FLOAT).unwrap();

    let location = particle.get_location();
    let coords = particle.coordinate;
    let direction_cosine = particle.direction_cosine.clone();

    let mut facet_coords: [MCVector<T>; N_POINTS_PER_FACET] = Default::default();
    let mut iteration: usize = 0;
    let mut move_factor: T = FromPrimitive::from_f64(0.5 * SMALL_FLOAT).unwrap();

    let num_facets_per_cell: usize = domain.mesh.cell_connectivity[location.cell].facet.len();

    loop {
        let tmp: T = FromPrimitive::from_f64(1e-16).unwrap();
        let plane_tolerance: T =
            tmp * (coords.x * coords.x + coords.y * coords.y + coords.z * coords.z);

        let mut distance_to_facet: [MCDistanceToFacet<T>; 24] = [MCDistanceToFacet::default(); 24]; // why 24? == numfacetpercell?

        // too lazy to replace by an iterator checking for facet_normal_dot_dcos > 0
        for facet_idx in 0..num_facets_per_cell {
            distance_to_facet[facet_idx].distance = huge_f;

            let plane = &domain.mesh.cell_geometry[location.cell][facet_idx];

            let facet_normal_dot_dcos: T = plane.a * direction_cosine.alpha
                + plane.b * direction_cosine.beta
                + plane.c * direction_cosine.gamma;

            if facet_normal_dot_dcos <= zero() {
                continue;
            }

            // Mesh-dependent code
            let points = domain.mesh.cell_connectivity[location.cell].facet[facet_idx].point;
            facet_coords[0] = domain.mesh.node[points[0] as usize];
            facet_coords[1] = domain.mesh.node[points[1] as usize];
            facet_coords[2] = domain.mesh.node[points[2] as usize];

            let t: T = mct_nf_3dg_dist_to_segment(
                plane_tolerance,
                facet_normal_dot_dcos,
                *plane,
                &facet_coords,
                &coords,
                &direction_cosine,
                false,
            );

            distance_to_facet[facet_idx].distance = t;
        }

        let mut retry: bool = false;

        let nearest_facet = mct_nf_find_nearest(
            particle,
            domain,
            &location,
            &mut iteration,
            &mut move_factor,
            num_facets_per_cell,
            &distance_to_facet,
            &mut retry,
        );

        if !retry {
            return nearest_facet;
        }
    }
}

/// Returns the volume defined by `v0-v3`, `v1-v3`, `v2-v3` using
/// vectorial operations.
fn mct_cell_volume_3dg_vector_tetdet<T: Float + FromPrimitive>(
    v0: &MCVector<T>,
    v1: &MCVector<T>,
    v2: &MCVector<T>,
    v3: &MCVector<T>,
) -> T {
    let tmp0 = *v0 - *v3;
    let tmp1 = *v1 - *v3;
    let tmp2 = *v2 - *v3;

    tmp0.dot(&tmp1.cross(&tmp2)) // should be the same as original code
}

fn mct_nf_3dg_move_particle<T: Float + FromPrimitive>(
    domain: &MCDomain<T>,
    location: &MCLocation,
    coord: &mut MCVector<T>,
    move_factor: T,
) {
    let move_to = cell_position_3dg(domain, location.cell);
    /*
    coord.x = coord.x + move_factor * (move_to.x - coord.x);
    coord.y = coord.y + move_factor * (move_to.y - coord.y);
    coord.z = coord.z + move_factor * (move_to.z - coord.z);
    */
    *coord += (move_to - *coord)*move_factor; 
}

/// delete num_facets_per_cell ?
fn mct_nf_compute_nearest<T: Float + FromPrimitive>(
    num_facets_per_cell: usize,
    distance_to_facet: &[MCDistanceToFacet<T>],
) -> MCNearestFacet<T> {
    let huge_f: T = FromPrimitive::from_f64(HUGE_FLOAT).unwrap();
    let mut nearest_facet: MCNearestFacet<T> = Default::default();
    let mut nearest_negative_facet: MCNearestFacet<T> = MCNearestFacet {
        distance_to_facet: -huge_f,
        ..Default::default()
    };

    // determine the nearest facet
    (0..num_facets_per_cell).into_iter().for_each(|facet_idx| {
        if distance_to_facet[facet_idx].distance > zero() {
            if distance_to_facet[facet_idx].distance <= nearest_facet.distance_to_facet {
                nearest_facet.distance_to_facet = distance_to_facet[facet_idx].distance;
                nearest_facet.facet = facet_idx;
            }
        } else if distance_to_facet[facet_idx].distance > nearest_negative_facet.distance_to_facet {
            nearest_negative_facet.distance_to_facet = distance_to_facet[facet_idx].distance;
            nearest_negative_facet.facet = facet_idx;
        }
    });

    if (nearest_facet.distance_to_facet == huge_f)
        & (nearest_negative_facet.distance_to_facet != -huge_f)
    {
        nearest_facet.distance_to_facet = nearest_negative_facet.distance_to_facet;
        nearest_facet.facet = nearest_negative_facet.facet;
    }

    nearest_facet
}

/// Needs heavy refactoring; called in mct_nf_3dg
#[allow(clippy::too_many_arguments)]
fn mct_nf_find_nearest<T: Float + FromPrimitive>(
    particle: &mut MCParticle<T>,
    domain: &MCDomain<T>,
    location: &MCLocation,
    iteration: &mut usize,
    move_factor: &mut T,
    num_facets_per_cell: usize,
    distance_to_facet: &[MCDistanceToFacet<T>],
    retry: &mut bool,
) -> MCNearestFacet<T> {
    let nearest_facet = mct_nf_compute_nearest(num_facets_per_cell, distance_to_facet);
    let huge_f: T = FromPrimitive::from_f64(HUGE_FLOAT).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();
    let threshold: T = FromPrimitive::from_f64(1.0e-2).unwrap();

    let coord = &mut particle.coordinate;

    const MAX_ALLOWED_SEGMENTS: usize = 10000000;
    const MAX_ITERATION: usize = 1000;
    let max: T = FromPrimitive::from_usize(MAX_ALLOWED_SEGMENTS).unwrap();

    *retry = false;

    if particle.species != Species::Unknown {
        // take an option as arg and check if is_some ?
        if (nearest_facet.distance_to_facet == huge_f) & (*move_factor > zero::<T>())
            | ((particle.num_segments > max) & (nearest_facet.distance_to_facet <= zero()))
        {
            mct_nf_3dg_move_particle(domain, location, coord, *move_factor);
            *iteration += 1;
            *move_factor = *move_factor * two;

            if *move_factor > threshold {
                *move_factor = threshold;
            }

            if *iteration == MAX_ITERATION {
                *retry = false;
            } else {
                *retry = true;
            }
            // need to replace the magic value; enum/bool for validity?
            //location.facet = -1;
        }
    }
    nearest_facet
}

fn mct_facet_points_3dg<T: Float>(
    domain: &MCDomain<T>,
    cell: usize,
    facet: usize,
) -> [usize; N_POINTS_PER_FACET] {
    let mut res: [usize; N_POINTS_PER_FACET] = [0; N_POINTS_PER_FACET];

    (0..N_POINTS_PER_FACET).into_iter().for_each(|point_idx| {
        res[point_idx] = domain.mesh.cell_connectivity[cell].facet[facet].point[point_idx] as usize;
    });

    res
}

fn mct_nf_3dg_dist_to_segment<T: Float + FromPrimitive>(
    plane_tolerance: T,
    facet_normal_dot_dcos: T,
    plane: MCGeneralPlane<T>,
    facet_coords: &[MCVector<T>],
    coords: &MCVector<T>,
    d_cos: &DirectionCosine<T>,
    allow_enter: bool,
) -> T {
    let huge_f: T = FromPrimitive::from_f64(HUGE_FLOAT).unwrap();
    let pfive: T = FromPrimitive::from_f64(0.5).unwrap();
    let bounding_box_tolerance: T = FromPrimitive::from_f64(1e-9).unwrap();
    let numerator: T = plane.a * coords.x + plane.b * coords.y + plane.c * coords.z + plane.d;

    if !allow_enter & (numerator < zero()) & (numerator * numerator > plane_tolerance) {
        return huge_f;
    }

    let distance: T = numerator / facet_normal_dot_dcos;

    let intersection_pt: MCVector<T> = MCVector {
        x: coords.x + distance * d_cos.alpha,
        y: coords.y + distance * d_cos.beta,
        z: coords.z + distance * d_cos.gamma,
    };

    // if the point doesn't belong to the facet, returns huge_f
    macro_rules! belongs_or_return {
        ($axis: ident) => {
            let below: bool = (facet_coords[0].$axis
                > intersection_pt.$axis + bounding_box_tolerance)
                & (facet_coords[1].$axis > intersection_pt.$axis + bounding_box_tolerance)
                & (facet_coords[2].$axis > intersection_pt.$axis + bounding_box_tolerance);
            let above: bool = (facet_coords[0].$axis
                < intersection_pt.$axis - bounding_box_tolerance)
                & (facet_coords[1].$axis < intersection_pt.$axis - bounding_box_tolerance)
                & (facet_coords[2].$axis < intersection_pt.$axis - bounding_box_tolerance);
            if below | above {
                // doesn't belong
                return huge_f;
            }
        };
    }

    // scalar value of the cross product between AB & AC
    macro_rules! ab_cross_ac {
        ($ax: expr, $ay: expr, $bx: expr, $by: expr, $cx: expr, $cy: expr) => {
            ($bx - $ax) * ($cy - $ay) - ($by - $ay) * ($cx - $ax)
        };
    }

    let mut cross0: T = zero();
    let mut cross1: T = zero();
    let mut cross2: T = zero();

    if (plane.c < -pfive) | (plane.c > pfive) {
        belongs_or_return!(x);
        belongs_or_return!(y);
        // update cross; TODO:  check if we can replace it by a cross product using MCVector
        cross1 = ab_cross_ac!(
            facet_coords[0].x,
            facet_coords[0].y,
            facet_coords[1].x,
            facet_coords[1].y,
            intersection_pt.x,
            intersection_pt.y
        );
        cross2 = ab_cross_ac!(
            facet_coords[1].x,
            facet_coords[1].y,
            facet_coords[2].x,
            facet_coords[2].y,
            intersection_pt.x,
            intersection_pt.y
        );
        cross0 = ab_cross_ac!(
            facet_coords[2].x,
            facet_coords[2].y,
            facet_coords[0].x,
            facet_coords[0].y,
            intersection_pt.x,
            intersection_pt.y
        );
    } else if (plane.b < -pfive) | (plane.b > pfive) {
        belongs_or_return!(x);
        belongs_or_return!(z);
        // update cross
        cross1 = ab_cross_ac!(
            facet_coords[0].z,
            facet_coords[0].x,
            facet_coords[1].z,
            facet_coords[1].x,
            intersection_pt.z,
            intersection_pt.x
        );
        cross2 = ab_cross_ac!(
            facet_coords[1].z,
            facet_coords[1].x,
            facet_coords[2].z,
            facet_coords[2].x,
            intersection_pt.z,
            intersection_pt.x
        );
        cross0 = ab_cross_ac!(
            facet_coords[2].z,
            facet_coords[2].x,
            facet_coords[0].z,
            facet_coords[0].x,
            intersection_pt.z,
            intersection_pt.x
        );
    } else if (plane.a < -pfive) | (plane.a > pfive) {
        belongs_or_return!(z);
        belongs_or_return!(y);
        // update cross
        cross1 = ab_cross_ac!(
            facet_coords[0].y,
            facet_coords[0].z,
            facet_coords[1].y,
            facet_coords[1].z,
            intersection_pt.y,
            intersection_pt.z
        );
        cross2 = ab_cross_ac!(
            facet_coords[1].y,
            facet_coords[1].z,
            facet_coords[2].y,
            facet_coords[2].z,
            intersection_pt.y,
            intersection_pt.z
        );
        cross0 = ab_cross_ac!(
            facet_coords[2].y,
            facet_coords[2].z,
            facet_coords[0].y,
            facet_coords[0].z,
            intersection_pt.y,
            intersection_pt.z
        );
    }

    let cross_tolerance: T = bounding_box_tolerance * (cross0 + cross1 + cross2).abs();

    if ((cross0 > -cross_tolerance) & (cross1 > -cross_tolerance) & (cross2 > -cross_tolerance))
        | ((cross0 < cross_tolerance) & (cross1 < cross_tolerance) & (cross2 < cross_tolerance))
    {
        return distance;
    }
    huge_f
}
