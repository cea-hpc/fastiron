//! Core of the particle tracking algorithm used by the simulation
//!
//! This module contains the function individually tracking particles during the
//! main simulation section.

use num::{one, zero};

use crate::{
    constants::CustomFloat,
    data::{send_queue::SendQueue, tallies::MCTallyEvent},
    montecarlo::MonteCarlo,
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
    mcco: &mut MonteCarlo<T>,
    base_particle: &mut MCBaseParticle<T>,
    particle_idx: usize,
    extra: &mut Vec<MCBaseParticle<T>>,
    send_queue: &mut SendQueue<T>,
) {
    // load particle, track it & update the original
    // next step is to refactor MCParticle / MCBaseParticle to lighten conversion between the types
    let mut particle = MCParticle::new(base_particle);

    // set age & time to census
    if particle.time_to_census <= zero() {
        particle.time_to_census += mcco.time_info.time_step;
    }
    if particle.age < zero() {
        particle.age = zero();
    }
    // update energy & task
    particle.energy_group = mcco.nuclear_data.get_energy_groups(particle.kinetic_energy);
    particle.task = 0; // useful?

    cycle_tracking_function(mcco, &mut particle, particle_idx, extra, send_queue);

    *base_particle = MCBaseParticle::new(&particle);
}

fn cycle_tracking_function<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    particle: &mut MCParticle<T>,
    particle_idx: usize,
    extra: &mut Vec<MCBaseParticle<T>>,
    send_queue: &mut SendQueue<T>,
) {
    let mut keep_tracking: bool;
    let tally_idx: usize = particle_idx % mcco.tallies.num_balance_replications as usize;
    let flux_tally_idx: usize = particle_idx % mcco.tallies.num_flux_replications as usize;

    loop {
        // compute event for segment & update # of segments
        let segment_outcome = outcome(mcco, particle, flux_tally_idx);
        mcco.tallies.balance_task[tally_idx].num_segments += 1; // atomic in original code

        particle.num_segments += one();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                keep_tracking = collision_event(mcco, particle, tally_idx, extra);
                if !keep_tracking {
                    particle.species = Species::Unknown;
                }
            }
            MCSegmentOutcome::FacetCrossing => {
                let facet_crossing_type = facet_crossing_event(particle, mcco, send_queue);

                keep_tracking = match facet_crossing_type {
                    MCTallyEvent::FacetCrossingTransitExit => true,
                    MCTallyEvent::FacetCrossingEscape => {
                        // atomic in original code
                        mcco.tallies.balance_task[tally_idx].escape += 1;
                        particle.last_event = MCTallyEvent::FacetCrossingEscape;
                        particle.species = Species::Unknown;
                        false
                    }
                    MCTallyEvent::FacetCrossingReflection => {
                        reflect_particle(mcco, particle);
                        true
                    }
                    _ => {
                        // transit to off-cluster domain
                        particle.species = Species::Unknown;
                        false
                    }
                };
            }
            MCSegmentOutcome::Census => {
                // atomic in original code
                mcco.tallies.balance_task[tally_idx].census += 1;
                // we're done tracking the particle FOR THIS STEP
                keep_tracking = false;
            }
            MCSegmentOutcome::Initialize => unreachable!(),
        }

        if !keep_tracking {
            break;
        }
    }
}
