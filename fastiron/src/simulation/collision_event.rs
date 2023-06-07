//! Event-specific code for particles colliding
//!
//! This module contains code that process particles undergoing a collision
//! from beginning to end. Note that _collision_ refers to reaction with the
//! particle's environment, not in-between particles.

use num::{zero, FromPrimitive};

use crate::{
    constants::CustomFloat,
    data::{
        nuclear_data::ReactionType,
        tallies::{Balance, TalliedEvent},
    },
    montecarlo::MonteCarloData,
    particles::{mc_particle::MCParticle, particle_collection::ParticleCollection},
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
    balance: &mut Balance,
    mat_gid: usize,
    cell_nb_density: T,
    particle: &mut MCParticle<T>,
    extra: &mut ParticleCollection<T>,
) -> bool {
    // ==========================
    // Pick an isotope & reaction

    let mut current_xsection: T = particle.get_current_xs();
    let mut reaction = None;

    while current_xsection >= zero() {
        for isotope in &mcdata.material_database.mat[mat_gid].iso {
            let atom_fraction = isotope.atom_fraction;
            for curr_reaction in &mcdata.nuclear_data.isotopes[isotope.gid][0].reactions {
                if (atom_fraction == zero()) | (cell_nb_density == zero()) {
                    current_xsection -= FromPrimitive::from_f64(1e-20).unwrap();
                } else {
                    current_xsection -= atom_fraction
                        * cell_nb_density
                        * curr_reaction.cross_section[particle.energy_group];
                }
                if current_xsection < zero() {
                    reaction = Some(curr_reaction);
                    break;
                }
            }
            if current_xsection < zero() {
                break;
            }
        }
    }
    assert!(reaction.is_some());
    let reaction = reaction.unwrap();

    let mat_mass = mcdata.material_database.mat[mat_gid].mass;

    // ================
    // Do the collision
    //
    // number of particles resulting from the collision, including the original
    // e.g. zero means the original particle was absorbed or invalidated in some way
    let n_out = particle.sample_collision(reaction, mat_mass, extra);

    //====================
    // Tally the collision

    balance[TalliedEvent::Collision] += 1;
    match reaction.reaction_type {
        ReactionType::Scatter => {
            balance[TalliedEvent::Scatter] += 1;
        }
        ReactionType::Absorption => {
            balance[TalliedEvent::Absorb] += 1;
        }
        ReactionType::Fission => {
            balance[TalliedEvent::Fission] += 1;
            balance[TalliedEvent::Produce] += n_out as u64;
        }
    };

    n_out >= 1
}
