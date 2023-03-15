use num::one;

use crate::{
    collision_event::collision_event,
    constants::CustomFloat,
    mc::{
        mc_base_particle::Species,
        mc_facet_crossing_event::facet_crossing_event,
        mc_particle::MCParticle,
        mc_segment_outcome::{outcome, MCSegmentOutcome},
        mc_utils::load_particle,
        mct::reflect_particle,
    },
    montecarlo::MonteCarlo,
    tallies::MCTallyEvent,
};

/// Main steps of the CycleTracking sections
pub fn cycle_tracking_guts<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    particle_idx: usize,
    processed_num: &mut usize,
    processing_vault_idx: usize,
    processed_vault_idx: usize,
) {
    let processing_vault = &mcco.particle_vault_container.processing_vaults[processing_vault_idx];
    //println!("processing particle #{particle_idx}");

    if let Some(mut particle) =
        load_particle(processing_vault, particle_idx, mcco.time_info.time_step)
    {
        particle.energy_group = mcco.nuclear_data.get_energy_groups(particle.kinetic_energy);
        particle.task = 0;

        cycle_tracking_function(
            mcco,
            &mut particle,
            particle_idx,
            processing_vault_idx,
            processed_vault_idx,
        );

        //mcco.particle_vault_container.processing_vaults[processing_vault_idx]
        //    .invalidate_particle(particle_idx);
        mcco.particle_vault_container
            .set_as_processed(processing_vault_idx, particle_idx);
        //println!("invalidated particle #{particle_idx}");
        *processed_num += 1;
    }
}

/// Computations of the CycleTracking sections
pub fn cycle_tracking_function<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    particle: &mut MCParticle<T>,
    particle_idx: usize,
    processing_vault_idx: usize,
    processed_vault_idx: usize,
) {
    let mut keep_tracking: bool;
    let tally_idx: usize = particle_idx % mcco.tallies.num_balance_replications as usize;
    let flux_tally_idx: usize = particle_idx % mcco.tallies.num_flux_replications as usize;
    //let cell_tally_idx: usize = particle_idx % mcco.tallies.num_cell_tally_replications as usize;

    loop {
        let segment_outcome = outcome(mcco, particle, flux_tally_idx);
        //println!("Seg outcome: {segment_outcome:?}");
        // atomic in original code
        mcco.tallies.balance_task[tally_idx].num_segments += 1;

        particle.num_segments += one();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                keep_tracking = collision_event(mcco, particle, tally_idx)
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
                        particle.species = Species::Unknown;
                        false
                    }
                    MCTallyEvent::FacetCrossingReflection => {
                        reflect_particle(mcco, particle);
                        true
                    }
                    _ => false,
                }
            }
            MCSegmentOutcome::Census => {
                let processing_vault =
                    &mut mcco.particle_vault_container.processing_vaults[processing_vault_idx];
                let processed_vault =
                    &mut mcco.particle_vault_container.processed_vaults[processed_vault_idx];

                // set the particle as processed, i.e. transfer it from processing to processed vault
                processed_vault.push_particle(particle.clone());
                processing_vault.invalidate_particle(particle_idx);

                // atomic in original code
                mcco.tallies.balance_task[tally_idx].census += 1;
                keep_tracking = false
            }
            MCSegmentOutcome::Initialize => unreachable!(),
        }

        if !keep_tracking {
            break;
        }
    }
}
