use num::one;

use crate::{
    constants::CustomFloat,
    mc::{mc_particle::MCParticle, mc_utils::load_particle},
    montecarlo::MonteCarlo,
    simulation::{mc_facet_crossing_event::facet_crossing_event, mct::reflect_particle},
    tallies::MCTallyEvent,
};

use super::{
    collision_event::collision_event,
    mc_segment_outcome::{outcome, MCSegmentOutcome},
};

/// Main steps of the CycleTracking section.
pub fn cycle_tracking_guts<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    particle_idx: usize,
    processing_vault_idx: usize,
) {
    // A major change to be done is to do every computation using a reference to the particles
    // instead of loading a copy & overwriting it; This would also mean propagating the
    // "keep_tracking" result to the top level functions where it can be used to update the
    // particle's status accordingly
    if let Some(mut particle) = load_particle(
        &mcco.particle_vault_container.processing_vaults[processing_vault_idx],
        particle_idx,
        mcco.time_info.time_step,
    ) {
        particle.energy_group = mcco.nuclear_data.get_energy_groups(particle.kinetic_energy);
        particle.task = 0;

        let keep_tracking_next_cycle =
            cycle_tracking_function(mcco, &mut particle, particle_idx, processing_vault_idx);

        // necessary overwrite
        mcco.particle_vault_container.processing_vaults[processing_vault_idx]
            .put_particle(particle.clone(), particle_idx);

        // These functions operate using indexes, i.e. the version of the particle that is
        // in the vault, not the copy we loaded & updated, hence the overwrite above
        if keep_tracking_next_cycle {
            mcco.particle_vault_container
                .set_as_processed(processing_vault_idx, particle_idx);
        } else {
            mcco.particle_vault_container.processing_vaults[processing_vault_idx]
                .invalidate_particle(particle_idx);
        }
    }
}

/// Computations of the CycleTracking section
pub fn cycle_tracking_function<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    particle: &mut MCParticle<T>,
    particle_idx: usize,
    processing_vault_idx: usize,
) -> bool {
    let mut keep_tracking: bool;
    let mut keep_tracking_next_cycle: bool;
    let tally_idx: usize = particle_idx % mcco.tallies.num_balance_replications as usize;
    let flux_tally_idx: usize = particle_idx % mcco.tallies.num_flux_replications as usize;

    loop {
        // compute event for segment & update # of segments
        let segment_outcome = outcome(mcco, particle, flux_tally_idx);
        mcco.tallies.balance_task[tally_idx].num_segments += 1; // atomic in original code

        particle.num_segments += one();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                keep_tracking = collision_event(mcco, particle, tally_idx);
                keep_tracking_next_cycle = keep_tracking;
            }
            MCSegmentOutcome::FacetCrossing => {
                let facet_crossing_type =
                    facet_crossing_event(particle, mcco, particle_idx, processing_vault_idx);

                keep_tracking = match facet_crossing_type {
                    MCTallyEvent::FacetCrossingTransitExit => true,
                    MCTallyEvent::FacetCrossingEscape => {
                        // atomic in original code
                        mcco.tallies.balance_task[tally_idx].escape += 1;
                        particle.last_event = MCTallyEvent::FacetCrossingEscape;
                        false
                    }
                    MCTallyEvent::FacetCrossingReflection => {
                        reflect_particle(mcco, particle);
                        true
                    }
                    _ => false, // transit to off-cluster domain
                };

                keep_tracking_next_cycle = keep_tracking;
            }
            MCSegmentOutcome::Census => {
                // atomic in original code
                mcco.tallies.balance_task[tally_idx].census += 1;

                // we're done tracking the particle FOR THIS STEP
                keep_tracking = false;
                keep_tracking_next_cycle = true;
            }
            MCSegmentOutcome::Initialize => unreachable!(),
        }

        if !keep_tracking {
            break;
        }
    }

    keep_tracking_next_cycle
}
