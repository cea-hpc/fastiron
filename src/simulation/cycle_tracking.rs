//! Core of the particle tracking algorithm used by the simulation
//!
//! This module contains the function individually tracking particles during the
//! main simulation section.

use num::{one, zero};

use crate::{
    constants::CustomFloat,
    data::{send_queue::SendQueue, tallies::MCTallyEvent},
    montecarlo::{MonteCarloData, MonteCarloUnit},
    particles::{
        mc_base_particle::{MCBaseParticle, Species},
        mc_particle::MCParticle,
    },
    simulation::{mc_facet_crossing_event::facet_crossing_event, mct::reflect_particle},
};

use super::{
    collision_event::collision_event,
    mc_segment_outcome::{outcome, MCSegmentOutcome},
};

/// Main steps of the `CycleTracking` section.
///
/// The particle at the specified index is loaded, tracked and updated accordingly.
/// Depeding on the outcome of the tracking, it is either set as processed or
/// invalidated.
pub fn cycle_tracking_guts<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    base_particle: &mut MCBaseParticle<T>,
    extra: &mut Vec<MCBaseParticle<T>>,
    send_queue: &mut SendQueue<T>,
) {
    // load particle, track it & update the original
    // next step is to refactor MCParticle / MCBaseParticle to lighten conversion between the types
    let mut particle = MCParticle::new(base_particle);

    // set age & time to census
    if particle.base_particle.time_to_census <= zero() {
        particle.base_particle.time_to_census += mcdata.params.simulation_params.dt;
    }
    if particle.base_particle.age < zero() {
        particle.base_particle.age = zero();
    }
    // update energy & task
    particle.energy_group = mcdata
        .nuclear_data
        .get_energy_groups(particle.base_particle.kinetic_energy);

    cycle_tracking_function(mcdata, mcunit, &mut particle, extra, send_queue);

    *base_particle = MCBaseParticle::new(&particle);
}

fn cycle_tracking_function<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    particle: &mut MCParticle<T>,
    extra: &mut Vec<MCBaseParticle<T>>,
    send_queue: &mut SendQueue<T>,
) {
    let mut keep_tracking: bool;

    loop {
        // compute event for segment & update # of segments
        let segment_outcome = outcome(mcdata, mcunit, particle);
        mcunit.tallies.balance_cycle.num_segments += 1; // atomic in original code

        particle.base_particle.num_segments += one();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                let mat_gid = mcunit.domain[particle.base_particle.domain].cell_state
                    [particle.base_particle.cell]
                    .material;
                let cell_nb_density = mcunit.domain[particle.base_particle.domain].cell_state
                    [particle.base_particle.cell]
                    .cell_number_density;

                keep_tracking = collision_event(
                    mcdata,
                    mat_gid,
                    cell_nb_density,
                    particle,
                    extra,
                    &mut mcunit.tallies.balance_cycle,
                );
                if !keep_tracking {
                    particle.base_particle.species = Species::Unknown;
                }
            }
            MCSegmentOutcome::FacetCrossing => {
                let facet_adjacency = &mcunit.domain[particle.base_particle.domain]
                    .mesh
                    .cell_connectivity[particle.base_particle.cell]
                    .facet[particle.facet]
                    .subfacet;

                facet_crossing_event(particle, facet_adjacency);

                keep_tracking = match particle.base_particle.last_event {
                    MCTallyEvent::FacetCrossingTransitExit => true,
                    MCTallyEvent::FacetCrossingEscape => {
                        // atomic in original code
                        mcunit.tallies.balance_cycle.escape += 1;
                        particle.base_particle.last_event = MCTallyEvent::FacetCrossingEscape;
                        particle.base_particle.species = Species::Unknown;
                        false
                    }
                    MCTallyEvent::FacetCrossingReflection => {
                        // plane on which particle is reflected
                        let plane = &mcunit.domain[particle.base_particle.domain]
                            .mesh
                            .cell_geometry[particle.base_particle.cell][particle.facet];

                        reflect_particle(particle, plane);
                        true
                    }
                    MCTallyEvent::FacetCrossingCommunication => {
                        // get destination neighbor
                        let neighbor_rank: usize = mcunit.domain
                            [facet_adjacency.current.domain.unwrap()]
                        .mesh
                        .nbr_rank[facet_adjacency.neighbor_index.unwrap()];
                        // add to sendqueue
                        send_queue.push(neighbor_rank, particle);
                        particle.base_particle.species = Species::Unknown;
                        false
                    }
                    _ => unreachable!(), // other values are for collision & census
                };
            }
            MCSegmentOutcome::Census => {
                // atomic in original code
                mcunit.tallies.balance_cycle.census += 1;
                // we're done tracking the particle FOR THIS STEP
                keep_tracking = false;
            }
        }

        if !keep_tracking {
            break;
        }
    }
}
