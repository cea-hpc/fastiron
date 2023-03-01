use core::panic;
use std::{fmt::Display};

use num::{Float, FromPrimitive, zero};

use crate::{montecarlo::MonteCarlo, physical_constants::{HUGE_FLOAT, SMALL_FLOAT, TINY_FLOAT}, macro_cross_section::weighted_macroscopic_cross_section, mc::{mc_rng_state::rng_sample, mc_nearest_facet::MCNearestFacet, mct::nearest_facet}, direction_cosine::DirectionCosine, tallies::MCTallyEvent};

use super::mc_particle::MCParticle;

/// Enum representing the outcome of the current segment.
#[derive(Debug, Clone, Copy)]
pub enum MCSegmentOutcome {
    Initialize = -1,
    Collision = 0,
    FacetCrossing = 1,
    Census = 2,
}

/// Enum representing the action to take after a particle collides.
#[derive(Debug)]
pub enum MCCollisionEventReturn {
    StopTracking = 0,
    ContinueTracking = 1,
    ContinueCollision = 2,
}

/// Computes the outcome of the current segment for a given particle.
pub fn outcome<T: Float + FromPrimitive + Display>(
    mcco: &mut MonteCarlo<T>,
    mc_particle: &mut MCParticle<T>,
    flux_tally_idx: usize,
) -> MCSegmentOutcome {
    // initialize distances and constants
    const N_EVENTS: usize = 3;
    let one: T = FromPrimitive::from_f64(1.0).unwrap();
    let ten: T = FromPrimitive::from_f64(10.0).unwrap();
    let huge_f: T = FromPrimitive::from_f64(HUGE_FLOAT).unwrap();
    let small_f: T = FromPrimitive::from_f64(SMALL_FLOAT).unwrap();
    let tiny_f: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
    let mut distance = [huge_f; N_EVENTS];

    let particle_speed = mc_particle.velocity.length();

    let mut force_collision = false;
    if mc_particle.num_mean_free_paths < zero() {
        force_collision = true;
        mc_particle.num_mean_free_paths = small_f;
    }

    // randomly determines the distance to the next collision
    // based upon the current cell data
    let macroscopic_total_xsection = weighted_macroscopic_cross_section(mcco, mc_particle.domain, mc_particle.cell, mc_particle.energy_group);

    mc_particle.total_cross_section = macroscopic_total_xsection;
    if macroscopic_total_xsection.abs() < tiny_f {
        mc_particle.mean_free_path = huge_f;
    } else {
        mc_particle.mean_free_path = one / macroscopic_total_xsection;
    }

    if mc_particle.num_mean_free_paths.abs() < tiny_f {
        let rdm_number: T = rng_sample(&mut mc_particle.random_number_seed);
        mc_particle.num_mean_free_paths = -one*rdm_number.ln();
    }

    // sets distance to collision, nearest facet and census

    // collision
    if force_collision {
        distance[MCSegmentOutcome::Collision as usize] = small_f;
    } else {
        distance[MCSegmentOutcome::Collision as usize] = mc_particle.num_mean_free_paths * mc_particle.mean_free_path;
    }
    // census
    distance[MCSegmentOutcome::Census as usize] = particle_speed*mc_particle.mean_free_path;
    // nearest facet
    let distance_threshold: T = ten * huge_f;
    let current_best_distance: T = huge_f; // useful?

    let d_cosine: &DirectionCosine<T> = &mc_particle.direction_cosine;
    let new_segment: bool = (mc_particle.num_segments.abs() < tiny_f) || (mc_particle.last_event == MCTallyEvent::Collision);

    let nearest_facet: MCNearestFacet<T> = nearest_facet(mc_particle, &mc_particle.get_location(), &mc_particle.coordinate, d_cosine, distance_threshold, current_best_distance, new_segment, mcco);
    mc_particle.normal_dot = nearest_facet.dot_product;

    distance[MCSegmentOutcome::FacetCrossing as usize] = nearest_facet.distance_to_facet;
    
    // exit if the tracker failed to bound the particle's volume
    if mc_particle.last_event == MCTallyEvent::FacetCrossingTrackingError {
        return MCSegmentOutcome::FacetCrossing;
    }
    // force a collision if needed
    if force_collision {
        distance[MCSegmentOutcome::FacetCrossing as usize] = huge_f;
        distance[MCSegmentOutcome::Census as usize] = huge_f;
        distance[MCSegmentOutcome::Collision as usize] = tiny_f;
    }

    let segment_outcome = find_min(&distance);

    if distance[segment_outcome as usize] < zero() {
        panic!()
    }
    mc_particle.segment_path_length = distance[segment_outcome as usize];
    mc_particle.num_mean_free_paths = mc_particle.num_mean_free_paths - mc_particle.segment_path_length/mc_particle.mean_free_path;

    // update the last event
    mc_particle.last_event = match segment_outcome {
        MCSegmentOutcome::Initialize => panic!(),
        MCSegmentOutcome::Collision => MCTallyEvent::Collision,
        MCSegmentOutcome::FacetCrossing => MCTallyEvent::FacetCrossingTransitExit,
        MCSegmentOutcome::Census => MCTallyEvent::Census,
    };

    // set the segment path length according to the minimum computed distance
    match segment_outcome {
        MCSegmentOutcome::Collision => mc_particle.num_mean_free_paths = zero(),
        MCSegmentOutcome::FacetCrossing => mc_particle.facet = nearest_facet.facet,
        MCSegmentOutcome::Census => mc_particle.time_to_census = zero::<T>().min(mc_particle.time_to_census),
        MCSegmentOutcome::Initialize => panic!(),
        
    }

    if force_collision {
        mc_particle.num_mean_free_paths = zero();
    }

    // skip tallies & early return if the path length is 0
    if mc_particle.segment_path_length.abs() < tiny_f {
        return segment_outcome
    }

    // move particle to the end of the segment
    let d_cos = mc_particle.direction_cosine.clone();
    mc_particle.move_particle(&d_cos, mc_particle.segment_path_length);

    // decrement time to census & increment age
    let segment_path_time = mc_particle.segment_path_length/particle_speed;
    mc_particle.time_to_census = mc_particle.time_to_census - segment_path_time;
    mc_particle.age = mc_particle.age + segment_path_time;
    if mc_particle.time_to_census < zero() {
        mc_particle.time_to_census = zero();
    }

    // update tallies
    mcco.tallies.tally_scalar_flux(mc_particle.segment_path_length*mc_particle.weight, mc_particle.domain, flux_tally_idx, mc_particle.cell, mc_particle.energy_group);

    segment_outcome
}

fn find_min<T: Float + FromPrimitive>(distance: &[T]) -> MCSegmentOutcome {
    let mut min_val: T = distance[0];
    let mut min_idx: usize = 0;
    (0..distance.len()).into_iter().for_each(|idx| {
        if distance[idx] < min_val {
            min_idx = idx;
            min_val = distance[idx];
        }
    });

    match min_idx {
        0 => MCSegmentOutcome::Collision,
        1 => MCSegmentOutcome::FacetCrossing,
        2 => MCSegmentOutcome::Census,
        _ => panic!(),
    }
}