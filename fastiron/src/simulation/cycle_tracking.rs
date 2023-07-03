//! Core of the particle tracking algorithm used by the simulation
//!
//! This module contains the function individually tracking particles during the
//! main simulation section.

use std::sync::atomic::Ordering;

use num::zero;

use crate::{
    constants::CustomFloat,
    data::tallies::{Balance, MCTallyEvent, TalliedEvent},
    montecarlo::{MonteCarloData, MonteCarloUnit},
    particles::{
        mc_particle::{MCParticle, Species},
        particle_collection::ParticleCollection,
    },
};

/// Main steps of the `CycleTracking` section.
///
/// The particle at the specified index is loaded, tracked and updated accordingly.
/// Depeding on the outcome of the tracking, it is either set as processed or
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
    if particle.age < zero() {
        particle.age = zero();
    }
    // update internal data
    particle.energy_group = mcdata
        .nuclear_data
        .get_energy_groups(particle.kinetic_energy);
    particle.mat_gid = mcunit.domain.cell_state[particle.cell].material;
    particle.cell_nb_density = mcunit.domain.cell_state[particle.cell].cell_number_density;

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
        particle.outcome(mcdata, mcunit);
        // update # of segments
        balance[TalliedEvent::NumSegments] += 1;

        // update scalar flux tally
        mcunit.tallies.scalar_flux_domain[(particle.cell, particle.energy_group)]
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                Some(x + particle.segment_path_length * particle.weight)
            })
            .unwrap();

        match particle.last_event {
            MCTallyEvent::Collision => {
                keep_tracking = particle.collision_event(mcdata, balance, extra);

                if !keep_tracking {
                    particle.species = Species::Unknown;
                }
            }
            MCTallyEvent::FacetCrossingTransitExit => keep_tracking = true,
            MCTallyEvent::Census => {
                balance[TalliedEvent::Census] += 1;
                // we're done tracking the particle FOR THIS STEP; Species stays valid
                keep_tracking = false;
            }
            MCTallyEvent::FacetCrossingEscape => {
                balance[TalliedEvent::Escape] += 1;
                particle.species = Species::Unknown;
                keep_tracking = false
            }
            MCTallyEvent::FacetCrossingReflection => {
                // plane on which particle is reflected
                particle.reflect();
                keep_tracking = true;
            }
            MCTallyEvent::FacetCrossingCommunication => unimplemented!(),
        }

        if !keep_tracking {
            break;
        }
    }
}
