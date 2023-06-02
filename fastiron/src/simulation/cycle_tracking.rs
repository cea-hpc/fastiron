//! Core of the particle tracking algorithm used by the simulation
//!
//! This module contains the function individually tracking particles during the
//! main simulation section.

use std::sync::{atomic::Ordering, Arc, Mutex};

use num::{one, zero};

use crate::{
    constants::CustomFloat,
    data::tallies::MCTallyEvent,
    montecarlo::{MonteCarloData, MonteCarloUnit},
    particles::{
        mc_particle::{MCParticle, Species},
        particle_collection::ParticleCollection,
    },
    simulation::{mc_facet_crossing_event::facet_crossing_event, mct::reflect_particle},
};

use super::{
    collision_event::collision_event,
    mc_segment_outcome::{outcome, MCSegmentOutcome},
};

//===================
// Sequential implems
//===================

/// Main steps of the `CycleTracking` section.
///
/// The particle at the specified index is loaded, tracked and updated accordingly.
/// Depeding on the outcome of the tracking, it is either set as processed or
/// invalidated.
pub fn cycle_tracking_guts<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &MonteCarloUnit<T>,
    particle: &mut MCParticle<T>,
    extra: &mut ParticleCollection<T>,
) {
    // set age & time to census
    if particle.time_to_census <= zero() {
        particle.time_to_census += mcdata.params.simulation_params.dt;
    }
    if particle.age < zero() {
        particle.age = zero();
    }
    // update energy & task
    particle.energy_group = mcdata
        .nuclear_data
        .get_energy_groups(particle.kinetic_energy);

    cycle_tracking_function(mcdata, mcunit, particle, extra);
}

fn cycle_tracking_function<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &MonteCarloUnit<T>,
    particle: &mut MCParticle<T>,
    extra: &mut ParticleCollection<T>,
) {
    let mut keep_tracking: bool;
    let tmp = Arc::new(Mutex::new(extra));

    loop {
        // compute event for segment
        let segment_outcome = outcome(mcdata, mcunit, particle);
        // update # of segments
        mcunit
            .tallies
            .balance_cycle
            .num_segments
            .fetch_add(1, Ordering::SeqCst);
        particle.num_segments += one();
        // update scalar flux of the cell
        mcunit.tallies.scalar_flux_domain[particle.domain].cell[particle.cell]
            [particle.energy_group]
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                Some(x + particle.segment_path_length * particle.weight)
            })
            .unwrap();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                let mat_gid = mcunit.domain.cell_state[particle.cell].material;
                let cell_nb_density = mcunit.domain.cell_state[particle.cell].cell_number_density;
                keep_tracking = collision_event(
                    mcdata,
                    &mcunit.tallies,
                    mat_gid,
                    cell_nb_density,
                    particle,
                    Arc::clone(&tmp),
                );

                particle.energy_group = mcdata
                    .nuclear_data
                    .get_energy_groups(particle.kinetic_energy);
                if !keep_tracking {
                    particle.species = Species::Unknown;
                }
            }
            MCSegmentOutcome::FacetCrossing => {
                // crossed facet data
                let facet_adjacency = &mcunit.domain.mesh.cell_connectivity[particle.cell].facet
                    [particle.facet]
                    .subfacet;

                facet_crossing_event(particle, facet_adjacency);

                keep_tracking = match particle.last_event {
                    // ~~~ on unit case
                    // on-unit transit
                    MCTallyEvent::FacetCrossingTransitExit => true,
                    // bound reflection
                    MCTallyEvent::FacetCrossingReflection => {
                        // plane on which particle is reflected
                        let plane =
                            &mcunit.domain.mesh.cell_geometry[particle.cell][particle.facet];

                        reflect_particle(particle, plane);
                        true
                    }
                    // ~~~ off unit case
                    // off-unit transit
                    MCTallyEvent::FacetCrossingCommunication => {
                        // get destination neighbor
                        unimplemented!()
                        /*
                        let neighbor_rank: usize = mcunit.domain
                            [facet_adjacency.current.domain.unwrap()]
                        .mesh
                        .nbr_rank[facet_adjacency.neighbor_index.unwrap()];
                        // add to sendqueue
                        send_queue.push(neighbor_rank, particle);
                        particle.species = Species::Unknown;
                        false*/
                    }
                    // bound escape
                    MCTallyEvent::FacetCrossingEscape => {
                        mcunit
                            .tallies
                            .balance_cycle
                            .escape
                            .fetch_add(1, Ordering::Relaxed);
                        particle.last_event = MCTallyEvent::FacetCrossingEscape;
                        particle.species = Species::Unknown;
                        false
                    }
                    // ~~~ other enum values are for collision & census, not facet crossing
                    _ => unreachable!(),
                };
            }
            MCSegmentOutcome::Census => {
                mcunit
                    .tallies
                    .balance_cycle
                    .census
                    .fetch_add(1, Ordering::Relaxed);
                // we're done tracking the particle FOR THIS STEP; Species stays valid
                keep_tracking = false;
            }
        }

        if !keep_tracking {
            break;
        }
    }
}

//=================
// Parallel implems
//=================

/// Main steps of the `CycleTracking` section.
///
/// The particle at the specified index is loaded, tracked and updated accordingly.
/// Depeding on the outcome of the tracking, it is either set as processed or
/// invalidated.
pub fn par_cycle_tracking_guts<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &MonteCarloUnit<T>,
    particle: &mut MCParticle<T>,
    extra: Arc<Mutex<&mut ParticleCollection<T>>>,
) {
    // set age & time to census
    if particle.time_to_census <= zero() {
        particle.time_to_census += mcdata.params.simulation_params.dt;
    }
    if particle.age < zero() {
        particle.age = zero();
    }
    // update energy & task
    particle.energy_group = mcdata
        .nuclear_data
        .get_energy_groups(particle.kinetic_energy);

    par_cycle_tracking_function(mcdata, mcunit, particle, extra);
}

fn par_cycle_tracking_function<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &MonteCarloUnit<T>,
    particle: &mut MCParticle<T>,
    extra: Arc<Mutex<&mut ParticleCollection<T>>>,
) {
    let mut keep_tracking: bool;

    loop {
        // compute event for segment
        let segment_outcome = outcome(mcdata, mcunit, particle);
        // update # of segments
        mcunit
            .tallies
            .balance_cycle
            .num_segments
            .fetch_add(1, Ordering::SeqCst);
        particle.num_segments += one();
        // update scalar flux tally
        mcunit.tallies.scalar_flux_domain[particle.domain].cell[particle.cell]
            [particle.energy_group]
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                Some(x + particle.segment_path_length * particle.weight)
            })
            .unwrap();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                let mat_gid = mcunit.domain.cell_state[particle.cell].material;
                let cell_nb_density = mcunit.domain.cell_state[particle.cell].cell_number_density;

                keep_tracking = collision_event(
                    mcdata,
                    &mcunit.tallies,
                    mat_gid,
                    cell_nb_density,
                    particle,
                    Arc::clone(&extra),
                );

                particle.energy_group = mcdata
                    .nuclear_data
                    .get_energy_groups(particle.kinetic_energy);
                if !keep_tracking {
                    particle.species = Species::Unknown;
                }
            }
            MCSegmentOutcome::FacetCrossing => {
                // crossed facet data
                let facet_adjacency = &mcunit.domain.mesh.cell_connectivity[particle.cell].facet
                    [particle.facet]
                    .subfacet;

                facet_crossing_event(particle, facet_adjacency);

                keep_tracking = match particle.last_event {
                    // ~~~ on unit case
                    // on-unit transit
                    MCTallyEvent::FacetCrossingTransitExit => true,
                    // bound reflection
                    MCTallyEvent::FacetCrossingReflection => {
                        // plane on which particle is reflected
                        let plane =
                            &mcunit.domain.mesh.cell_geometry[particle.cell][particle.facet];

                        reflect_particle(particle, plane);
                        true
                    }
                    // ~~~ off unit case
                    // off-unit transit
                    MCTallyEvent::FacetCrossingCommunication => {
                        // get destination neighbor
                        unimplemented!()
                        /*
                        let neighbor_rank: usize = mcunit.domain
                            [facet_adjacency.current.domain.unwrap()]
                        .mesh
                        .nbr_rank[facet_adjacency.neighbor_index.unwrap()];
                        // add to sendqueue
                        send_queue.lock().unwrap().push(neighbor_rank, particle);
                        particle.species = Species::Unknown;
                        false
                        */
                    }
                    // bound escape
                    MCTallyEvent::FacetCrossingEscape => {
                        mcunit
                            .tallies
                            .balance_cycle
                            .escape
                            .fetch_add(1, Ordering::Relaxed);
                        particle.last_event = MCTallyEvent::FacetCrossingEscape;
                        particle.species = Species::Unknown;
                        false
                    }
                    // ~~~ other enum values are for collision & census, not facet crossing
                    _ => unreachable!(),
                };
            }
            MCSegmentOutcome::Census => {
                mcunit
                    .tallies
                    .balance_cycle
                    .census
                    .fetch_add(1, Ordering::Relaxed);
                // we're done tracking the particle FOR THIS STEP; Species stays valid
                keep_tracking = false;
            }
        }

        if !keep_tracking {
            break;
        }
    }
}
