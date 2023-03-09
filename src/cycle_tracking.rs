use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::AddAssign,
    rc::Rc,
};

use num::{one, Float, FromPrimitive};

use crate::{
    collision_event::collision_event,
    mc::{
        mc_base_particle::Species,
        mc_facet_crossing_event::facet_crossing_event,
        mc_particle::MCParticle,
        mc_segment_outcome::{outcome, MCSegmentOutcome},
        mc_utils::load_particle,
        mct::reflect_particle,
    },
    montecarlo::MonteCarlo,
    particle_vault::ParticleVault,
    tallies::MCTallyEvent,
};

/// Main steps of the CycleTracking sections
pub fn cycle_tracking_guts<T: Float + FromPrimitive + Display + Debug + AddAssign>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    particle_idx: usize,
    processing_vault: &mut ParticleVault<T>,
    processed_vault: &mut ParticleVault<T>,
) {
    let mut particle = load_particle(&mcco.borrow(), processing_vault, particle_idx);
    particle.task = 0;

    cycle_tracking_function(
        mcco,
        &mut particle,
        particle_idx,
        processing_vault,
        processed_vault,
    );

    processing_vault.invalidate_particle(particle_idx);
}

/// Computations of the CycleTracking sections
pub fn cycle_tracking_function<T: Float + FromPrimitive + Display + Debug + AddAssign>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    particle: &mut MCParticle<T>,
    particle_idx: usize,
    processing_vault: &mut ParticleVault<T>,
    processed_vault: &mut ParticleVault<T>,
) {
    let mut keep_tracking: bool;
    let tally_idx: usize = particle_idx % mcco.borrow().tallies.num_balance_replications as usize;
    let flux_tally_idx: usize = particle_idx % mcco.borrow().tallies.num_flux_replications as usize;
    //let cell_tally_idx: usize = particle_idx % mcco.borrow().tallies.num_cell_tally_replications as usize;

    loop {
        let segment_outcome = outcome(&mut mcco.borrow_mut(), particle, flux_tally_idx);

        // atomic in original code
        mcco.borrow_mut().tallies.balance_task[tally_idx].num_segments += 1;

        particle.num_segments += one();

        match segment_outcome {
            MCSegmentOutcome::Collision => {
                keep_tracking = collision_event(&mut mcco.borrow_mut(), particle, tally_idx)
            }
            MCSegmentOutcome::FacetCrossing => {
                let facet_crossing_type = facet_crossing_event(
                    particle,
                    &mut mcco.borrow_mut(),
                    particle_idx,
                    processing_vault,
                );

                keep_tracking = match facet_crossing_type {
                    MCTallyEvent::FacetCrossingTransitExit => true,
                    MCTallyEvent::FacetCrossingEscape => {
                        // atomic in original code
                        mcco.borrow_mut().tallies.balance_task[tally_idx].escape += 1;
                        particle.last_event = MCTallyEvent::FacetCrossingEscape;
                        particle.species = Species::Unknown;
                        false
                    }
                    MCTallyEvent::FacetCrossingReflection => {
                        reflect_particle(&mcco.borrow(), particle);
                        true
                    }
                    _ => false,
                }
            }
            MCSegmentOutcome::Census => {
                processed_vault.push_particle(particle.clone());
                processing_vault.erase_swap_particles(particle_idx); //?
                                                                     // atomic in original code
                mcco.borrow_mut().tallies.balance_task[tally_idx].census += 1;
                keep_tracking = false
            }
            MCSegmentOutcome::Initialize => unreachable!(),
        }

        if !keep_tracking {
            break;
        }
    }
}
