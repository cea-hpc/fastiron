//! Core of the particle tracking algorithm used by the simulation
//!
//! This module contains the function individually tracking particles during the
//! main simulation section.

use std::sync::atomic::Ordering;

use num::{one, zero};

use crate::{
    constants::CustomFloat,
    data::tallies::{Balance, MCTallyEvent, TalliedEvent},
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

/// Main steps of the `CycleTracking` section.
///
/// The particle at the specified index is loaded, tracked and updated accordingly.
/// Depending on the outcome of the tracking, it is either set as processed or
/// invalidated.
pub fn cycle_tracking_guts<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &MonteCarloUnit<T>,
    particle: &mut MCParticle<T>,
    balance: &mut Balance,
    extra: &mut ParticleCollection<T>,
) {
    // set age & time to census
    if particle.time_to_census <= zero() {
        particle.time_to_census += mcdata.params.simulation_params.dt;
    }
    particle.age = particle.age.max(zero());
    // update energy & task
    particle.energy_group = mcdata
        .nuclear_data
        .get_energy_groups(particle.kinetic_energy);

    cycle_tracking_function(mcdata, mcunit, particle, balance, extra);
}

fn cycle_tracking_function<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &MonteCarloUnit<T>,
    particle: &mut MCParticle<T>,
    balance: &mut Balance,
    extra: &mut ParticleCollection<T>,
) {
    let mut keep_tracking: bool;

    loop {
        // compute event for segment
        let segment_outcome = outcome(mcdata, mcunit, particle);
        // update # of segments
        balance[TalliedEvent::NumSegments] += 1;
        particle.num_segments += one();
        // update scalar flux tally
        mcunit.tallies.scalar_flux_domain[(particle.cell, particle.energy_group)]
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                Some(x + particle.segment_path_length * particle.weight)
            })
            .unwrap();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                let mat_gid = mcunit.domain.cell_state[particle.cell].material;
                let cell_nb_density = mcunit.domain.cell_state[particle.cell].cell_number_density;

                keep_tracking =
                    collision_event(mcdata, balance, mat_gid, cell_nb_density, particle, extra);

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
                        unimplemented!()
                    }
                    // bound escape
                    MCTallyEvent::FacetCrossingEscape => {
                        balance[TalliedEvent::Escape] += 1;
                        particle.last_event = MCTallyEvent::FacetCrossingEscape;
                        particle.species = Species::Unknown;
                        false
                    }
                    // ~~~ other enum values are for collision & census, not facet crossing
                    _ => unreachable!(),
                };
            }
            MCSegmentOutcome::Census => {
                balance[TalliedEvent::Census] += 1;
                // we're done tracking the particle FOR THIS STEP; Species stays valid
                keep_tracking = false;
            }
        }

        if !keep_tracking {
            break;
        }
    }
}
