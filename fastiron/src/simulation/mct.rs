//! Code for spatial computations of the simulation
//!
//! This module contains function used to compute and manipulate data related
//! to a particle's coordinate and direction in the problem.

use num::{one, zero, FromPrimitive};

use crate::{
    constants::CustomFloat,
    data::mc_vector::MCVector,
    geometry::{
        facets::{MCGeneralPlane, MCNearestFacet},
        mc_domain::MCMeshDomain,
        N_FACETS_OUT, N_POINTS_INTERSEC, N_POINTS_PER_FACET,
    },
    particles::mc_particle::MCParticle,
    utils::mc_rng_state::rng_sample,
};

/// Computes which facet of the specified cell is nearest to the specified
/// coordinates.
///
/// The function uses the particle's direction to compute which facet is currently
/// the closest to the particle as well as the distance to this facet. The result is
/// used in order to assess which event the particle will undergo next, in this
/// case, a facet crossing. See [MCNearestFacet] for more information.
pub fn nearest_facet<T: CustomFloat>(
    particle: &mut MCParticle<T>,
    mesh: &MCMeshDomain<T>,
) -> MCNearestFacet<T> {
    let mut nearest_facet = mct_nf_3dg(particle, mesh);

    nearest_facet.distance_to_facet = nearest_facet.distance_to_facet.max(zero());
    assert!(nearest_facet.distance_to_facet <= T::huge_float());

    nearest_facet
}

/// Generates a random coordinate inside a polyhedral cell.
pub fn generate_coordinate_3dg<T: CustomFloat>(
    seed: &mut u64,
    mesh: &MCMeshDomain<T>,
    cell_idx: usize,
    cell_volume: T,
) -> MCVector<T> {
    let six: T = FromPrimitive::from_f64(6.0).unwrap();
    let one: T = one();

    let center: MCVector<T> = cell_position_3dg(mesh, cell_idx);

    let which_volume = rng_sample::<T>(seed) * six * cell_volume;

    let mut current_volume: T = zero();
    let mut points = [MCVector::default(); N_POINTS_PER_FACET];

    // find the facet to sample from
    for facet_idx in 0..N_FACETS_OUT {
        points = mesh.get_facet_coords(cell_idx, facet_idx);

        let subvolume = compute_volume(&points, &center);
        current_volume += subvolume;
        if current_volume >= which_volume {
            break;
        }
    }

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

    points[0] * r1 + points[1] * r2 + points[2] * r3 + center * r4
}

/// Returns a coordinate that represents the "center" of the cell.
pub fn cell_position_3dg<T: CustomFloat>(mesh: &MCMeshDomain<T>, cell_idx: usize) -> MCVector<T> {
    let mut coordinate: MCVector<T> = mesh.cell_connectivity[cell_idx]
        .point
        .map(|point_idx| mesh.node[point_idx])
        .iter()
        .sum();

    coordinate /= FromPrimitive::from_usize(N_POINTS_INTERSEC).unwrap();

    coordinate
}

/// Reflects a particle off a reflection-type boundary.
///
/// This function is called when a particle undergo a reflectionevent at the
/// boundary of the problem. Note that the reflection does not result in a
/// loss of energy.
pub fn reflect_particle<T: CustomFloat>(particle: &mut MCParticle<T>, plane: &MCGeneralPlane<T>) {
    let facet_normal: MCVector<T> = MCVector {
        x: plane.a,
        y: plane.b,
        z: plane.c,
    };

    let two: T = FromPrimitive::from_f64(2.0).unwrap();
    let dot: T = two * particle.direction.dot(&facet_normal);

    if dot > zero() {
        particle.direction -= facet_normal * dot;
    }
}

// ==============================
//       Private functions
// ==============================

fn mct_nf_3dg<T: CustomFloat>(
    particle: &mut MCParticle<T>,
    mesh: &MCMeshDomain<T>,
) -> MCNearestFacet<T> {
    let coords = particle.coordinate;
    let direction = particle.direction;

    let mut facet_coords: [MCVector<T>; N_POINTS_PER_FACET] = Default::default();
    let mut iteration: usize = 0;
    let mut move_factor: T = <T as FromPrimitive>::from_f64(0.5).unwrap() * T::small_float();

    let tmp: T = FromPrimitive::from_f64(1e-16).unwrap();
    let plane_tolerance: T =
        tmp * (coords.x * coords.x + coords.y * coords.y + coords.z * coords.z);

    loop {
        // the link between distances and facet idx is made implicitly through
        // array indexing
        let mut distance_to_facet: [T; N_FACETS_OUT] = [T::huge_float(); N_FACETS_OUT];
        let planes = &mesh.cell_geometry[particle.cell];
        distance_to_facet
            .iter_mut()
            .enumerate()
            .zip(planes.iter())
            .for_each(|((facet_idx, dist), plane)| {
                facet_coords = mesh.get_facet_coords(particle.cell, facet_idx);

                let numerator: T = -one::<T>()
                    * (plane.a * coords.x + plane.b * coords.y + plane.c * coords.z + plane.d);
                let facet_normal_dot_dcos: T =
                    plane.a * direction.x + plane.b * direction.y + plane.c * direction.z;

                if (facet_normal_dot_dcos <= zero())
                    | (numerator < zero()) & (numerator * numerator > plane_tolerance)
                {
                    return;
                }

                let distance = numerator / facet_normal_dot_dcos;
                let intersection_pt: MCVector<T> = coords + direction * distance;

                if mct_nf_3dg_dist_to_segment(&intersection_pt, plane, &facet_coords) {
                    *dist = distance;
                }
            });

        let nearest_facet = mct_nf_compute_nearest(distance_to_facet);
        let retry = check_nearest_validity(
            particle,
            mesh,
            &mut iteration,
            &mut move_factor,
            &nearest_facet,
        );

        if !retry {
            return nearest_facet;
        }
    }
}

/// Returns the volume defined by `v3v0`, `v3v1`, `v3v2` using
/// vectorial operations.
fn compute_volume<T: CustomFloat>(vertices: &[MCVector<T>], origin: &MCVector<T>) -> T {
    assert_eq!(vertices.len(), N_POINTS_PER_FACET);
    let tmp0 = vertices[0] - *origin;
    let tmp1 = vertices[1] - *origin;
    let tmp2 = vertices[2] - *origin;

    tmp0.dot(&tmp1.cross(&tmp2)) // should be the same as original code
}

fn mct_nf_compute_nearest<T: CustomFloat>(
    distance_to_facet: [T; N_FACETS_OUT],
) -> MCNearestFacet<T> {
    let huge_f: T = T::huge_float();
    let mut nearest_facet: MCNearestFacet<T> = Default::default();
    let mut nearest_negative_facet: MCNearestFacet<T> = MCNearestFacet {
        distance_to_facet: -huge_f,
        ..Default::default()
    };

    // determine the nearest facet
    distance_to_facet
        .iter()
        .enumerate()
        .for_each(|(facet_idx, dist)| {
            if *dist > zero() {
                if *dist <= nearest_facet.distance_to_facet {
                    nearest_facet.distance_to_facet = *dist;
                    nearest_facet.facet = facet_idx;
                }
            } else if *dist > nearest_negative_facet.distance_to_facet {
                nearest_negative_facet.distance_to_facet = *dist;
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

fn check_nearest_validity<T: CustomFloat>(
    particle: &mut MCParticle<T>,
    mesh: &MCMeshDomain<T>,
    iteration: &mut usize,
    move_factor: &mut T,
    nearest_facet: &MCNearestFacet<T>,
) -> bool {
    const MAX_ALLOWED_SEGMENTS: usize = 10000000;
    const MAX_ITERATION: usize = 1000;
    let max: T = FromPrimitive::from_usize(MAX_ALLOWED_SEGMENTS).unwrap();

    let coord = &mut particle.coordinate;

    if (nearest_facet.distance_to_facet == T::huge_float())
        | ((particle.num_segments > max) & (nearest_facet.distance_to_facet <= zero()))
    {
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let threshold: T = FromPrimitive::from_f64(1.0e-2).unwrap();

        // move coordinates towards cell center
        let move_to = cell_position_3dg(mesh, particle.cell);
        *coord += (move_to - *coord) * *move_factor;

        // keep track of the movement
        *iteration += 1;
        *move_factor = threshold.min(*move_factor * two);

        return *iteration != MAX_ITERATION;
    }
    false
}

fn mct_nf_3dg_dist_to_segment<T: CustomFloat>(
    intersection_pt: &MCVector<T>,
    plane: &MCGeneralPlane<T>,
    facet_coords: &[MCVector<T>],
) -> bool {
    let pfive: T = FromPrimitive::from_f64(0.5).unwrap();
    let bounding_box_tolerance: T = FromPrimitive::from_f64(1e-9).unwrap();

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
                return false;
            }
        };
    }

    // scalar value of the cross product between AB & AC
    macro_rules! ab_cross_ac {
        ($ax: expr, $ay: expr, $bx: expr, $by: expr, $cx: expr, $cy: expr) => {
            ($bx - $ax) * ($cy - $ay) - ($by - $ay) * ($cx - $ax)
        };
    }

    let crosses = if plane.c.abs() > pfive {
        belongs_or_return!(x);
        belongs_or_return!(y);
        // update cross; z elements
        [
            ab_cross_ac!(
                facet_coords[0].x,
                facet_coords[0].y,
                facet_coords[1].x,
                facet_coords[1].y,
                intersection_pt.x,
                intersection_pt.y
            ),
            ab_cross_ac!(
                facet_coords[1].x,
                facet_coords[1].y,
                facet_coords[2].x,
                facet_coords[2].y,
                intersection_pt.x,
                intersection_pt.y
            ),
            ab_cross_ac!(
                facet_coords[2].x,
                facet_coords[2].y,
                facet_coords[0].x,
                facet_coords[0].y,
                intersection_pt.x,
                intersection_pt.y
            ),
        ]
    } else if plane.b.abs() > pfive {
        belongs_or_return!(x);
        belongs_or_return!(z);
        // update cross; y elements
        [
            ab_cross_ac!(
                facet_coords[0].z,
                facet_coords[0].x,
                facet_coords[1].z,
                facet_coords[1].x,
                intersection_pt.z,
                intersection_pt.x
            ),
            ab_cross_ac!(
                facet_coords[1].z,
                facet_coords[1].x,
                facet_coords[2].z,
                facet_coords[2].x,
                intersection_pt.z,
                intersection_pt.x
            ),
            ab_cross_ac!(
                facet_coords[2].z,
                facet_coords[2].x,
                facet_coords[0].z,
                facet_coords[0].x,
                intersection_pt.z,
                intersection_pt.x
            ),
        ]
    } else if plane.a.abs() > pfive {
        belongs_or_return!(z);
        belongs_or_return!(y);
        // update cross; x elements
        [
            ab_cross_ac!(
                facet_coords[0].y,
                facet_coords[0].z,
                facet_coords[1].y,
                facet_coords[1].z,
                intersection_pt.y,
                intersection_pt.z
            ),
            ab_cross_ac!(
                facet_coords[1].y,
                facet_coords[1].z,
                facet_coords[2].y,
                facet_coords[2].z,
                intersection_pt.y,
                intersection_pt.z
            ),
            ab_cross_ac!(
                facet_coords[2].y,
                facet_coords[2].z,
                facet_coords[0].y,
                facet_coords[0].z,
                intersection_pt.y,
                intersection_pt.z
            ),
        ]
    } else {
        [zero(); 3]
    };

    let cross_tolerance: T = bounding_box_tolerance * (crosses[0] + crosses[1] + crosses[2]).abs();

    if ((crosses[0] > -cross_tolerance)
        & (crosses[1] > -cross_tolerance)
        & (crosses[2] > -cross_tolerance))
        | ((crosses[0] < cross_tolerance)
            & (crosses[1] < cross_tolerance)
            & (crosses[2] < cross_tolerance))
    {
        return true;
    }
    false
}
