//! Event-specific code for particles colliding
//!
//! This module contains code that process particles undergoing a collision
//! from beginning to end. Note that _collision_ refers to reaction with the
//! particle's environment, not in-between particles.

use std::sync::{atomic::Ordering, Arc, Mutex};

use num::zero;

use crate::{
    constants::CustomFloat,
    data::{nuclear_data::ReactionType, tallies::Tallies},
    montecarlo::MonteCarloData,
    particles::mc_particle::MCParticle,
    simulation::macro_cross_section::macroscopic_cross_section,
};

/// Transforms a given particle according to an internally drawn type of collision.
///
/// The function calls method from [`super::macro_cross_section`] module to pick
/// the reaction the particle will undergo (See [`ReactionType`]). The particle is then updated and the
/// collision tallied. Finally, particles are created / invalidated accordingly to
/// the picked reaction:
///
/// - Absorption reaction: the particle is invalidated.
/// - Fission reaction: offspring particles are created from the colliding one.
/// - Scattering reaction: no additional modifications occur.
pub fn collision_event<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    tallies: &Tallies<T>,
    mat_gid: usize,
    cell_nb_density: T,
    particle: &mut MCParticle<T>,
    extra: Arc<Mutex<&mut Vec<MCParticle<T>>>>,
) -> bool {
    // ==========================
    // Pick an isotope & reaction

    let mut current_xsection: T = particle.get_current_xs();

    // sort of a magic value but using an option seems to be overkill
    let mut selected_iso: usize = usize::MAX;
    let mut selected_unique_n: usize = usize::MAX;
    let mut selected_react: usize = usize::MAX;

    let n_iso: usize = mcdata.material_database.mat[mat_gid].iso.len();

    loop {
        for iso_idx in 0..n_iso {
            let unique_n: usize = mcdata.material_database.mat[mat_gid].iso[iso_idx].gid;
            let n_reactions: usize = mcdata.nuclear_data.get_number_reactions(unique_n);
            for reaction_idx in 0..n_reactions {
                current_xsection -= macroscopic_cross_section(
                    mcdata,
                    reaction_idx,
                    mat_gid,
                    cell_nb_density,
                    iso_idx,
                    particle.energy_group,
                );
                if current_xsection < zero() {
                    selected_iso = iso_idx;
                    selected_unique_n = unique_n;
                    selected_react = reaction_idx;
                    break;
                }
            }
            if current_xsection < zero() {
                break;
            }
        }
        if current_xsection < zero() {
            break;
        }
    }
    assert_ne!(selected_iso, usize::MAX); // sort of a magic value

    let mat_mass = mcdata.material_database.mat[mat_gid].mass;
    let reaction = &mcdata.nuclear_data.isotopes[selected_unique_n][0].reactions[selected_react];

    // ================
    // Do the collision
    //
    // number of particles resulting from the collision, including the original
    // e.g. zero means the original particle was absorbed or invalidated in some way
    let n_out = particle.sample_collision(reaction, mat_mass, extra);

    //====================
    // Tally the collision

    tallies
        .balance_cycle
        .collision
        .fetch_add(1, Ordering::Relaxed); // atomic in original code
    match reaction.reaction_type {
        ReactionType::Scatter => {
            tallies
                .balance_cycle
                .scatter
                .fetch_add(1, Ordering::Relaxed);
        }
        ReactionType::Absorption => {
            tallies.balance_cycle.absorb.fetch_add(1, Ordering::Relaxed);
        }
        ReactionType::Fission => {
            tallies
                .balance_cycle
                .fission
                .fetch_add(1, Ordering::Relaxed);
            tallies
                .balance_cycle
                .produce
                .fetch_add(n_out as u64, Ordering::Relaxed);
        }
    };

    n_out >= 1
}
