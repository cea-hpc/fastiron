//! Event-specific code for particles colliding
//!
//! This module contains code that process particles undergoing a collision
//! from beginning to end. Note that _collision_ refers to reaction with the
//! particle's environment, not in-between particles.

use num::FromPrimitive;

use crate::{
    constants::{
        physical::{LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY, PI},
        CustomFloat,
    },
    data::nuclear_data::ReactionType,
    montecarlo::MonteCarlo,
    particles::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle},
    simulation::macro_cross_section::macroscopic_cross_section,
    utils::mc_rng_state::{rng_sample, spawn_rn_seed},
};

fn update_trajectory<T: CustomFloat>(energy: T, angle: T, particle: &mut MCParticle<T>) {
    // constants
    let pi: T = FromPrimitive::from_f64(PI).unwrap();
    let c: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
    let nrm: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
    let one: T = FromPrimitive::from_f64(1.0).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();

    // value for update
    let cos_theta: T = angle;
    let sin_theta: T = (one - cos_theta * cos_theta).sqrt();
    let mut rdm_number: T = rng_sample(&mut particle.base_particle.random_number_seed);
    let phi = two * pi * rdm_number;
    let sin_phi: T = phi.sin();
    let cos_phi: T = phi.cos();
    let speed: T = c * (one - ((nrm * nrm) / ((energy + nrm) * (energy + nrm)))).sqrt();

    // update
    particle.base_particle.kinetic_energy = energy;
    particle
        .direction_cosine
        .rotate_3d_vector(sin_theta, cos_theta, sin_phi, cos_phi);
    particle.base_particle.velocity.x = speed * particle.direction_cosine.alpha;
    particle.base_particle.velocity.y = speed * particle.direction_cosine.beta;
    particle.base_particle.velocity.z = speed * particle.direction_cosine.gamma;
    rdm_number = rng_sample(&mut particle.base_particle.random_number_seed);
    particle.base_particle.num_mean_free_paths = -one * rdm_number.ln();
}

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
    mcco: &mut MonteCarlo<T>,
    particle: &mut MCParticle<T>,
    tally_idx: usize,
    extra: &mut Vec<MCBaseParticle<T>>,
) -> bool {
    let mat_gidx =
        mcco.domain[particle.base_particle.domain].cell_state[particle.base_particle.cell].material;

    // ==========================
    // Pick an isotope & reaction

    let rdm_number: T = rng_sample(&mut particle.base_particle.random_number_seed);
    let total_xsection: T = particle.total_cross_section;

    let mut current_xsection: T = total_xsection * rdm_number;

    // sort of a magic value but using an option seems to be overkill
    let mut selected_iso: usize = usize::MAX;
    let mut selected_unique_n: usize = usize::MAX;
    let mut selected_react: usize = usize::MAX;

    let n_iso: usize = mcco.material_database.mat[mat_gidx].iso.len();

    loop {
        for iso_idx in 0..n_iso {
            let unique_n: usize = mcco.material_database.mat[mat_gidx].iso[iso_idx].gid;
            let n_reactions: usize = mcco.nuclear_data.get_number_reactions(unique_n);
            for reaction_idx in 0..n_reactions {
                current_xsection -= macroscopic_cross_section(
                    mcco,
                    reaction_idx,
                    particle.base_particle.domain,
                    particle.base_particle.cell,
                    iso_idx,
                    particle.energy_group,
                );
                if current_xsection.is_sign_negative() {
                    selected_iso = iso_idx;
                    selected_unique_n = unique_n;
                    selected_react = reaction_idx;
                    break;
                }
            }
            if current_xsection.is_sign_negative() {
                break;
            }
        }
        if current_xsection.is_sign_negative() {
            break;
        }
    }
    assert_ne!(selected_iso, usize::MAX); // sort of a magic value

    // ================
    // Do the collision

    let mat_mass = mcco.material_database.mat[mat_gidx].mass;
    let (energy_out, angle_out) = mcco.nuclear_data.isotopes[selected_unique_n][0].reactions
        [selected_react]
        .sample_collision(
            particle.base_particle.kinetic_energy,
            mat_mass,
            &mut particle.base_particle.random_number_seed,
        );
    // number of particles resulting from the collision, including the original
    // e.g. zero means the original particle was absorbed or invalidated in some way
    let n_out = energy_out.len();

    // ===================
    // Tally the collision

    mcco.tallies.balance_task[tally_idx].collision += 1; // atomic in original code
    match mcco.nuclear_data.isotopes[selected_unique_n][0].reactions[selected_react].reaction_type {
        ReactionType::Scatter => mcco.tallies.balance_task[tally_idx].scatter += 1,
        ReactionType::Absorption => mcco.tallies.balance_task[tally_idx].absorb += 1,
        ReactionType::Fission => {
            mcco.tallies.balance_task[tally_idx].fission += 1;
            mcco.tallies.balance_task[tally_idx].produce += n_out as u64;
        }
        ReactionType::Undefined => panic!(),
    }

    // ================
    // Particle updates

    // Early return
    if n_out == 0 {
        return false;
    }

    // additional created particle
    if n_out > 1 {
        for secondary_idx in 1..n_out {
            let mut sec_particle = particle.clone();
            sec_particle.base_particle.random_number_seed =
                spawn_rn_seed::<T>(&mut particle.base_particle.random_number_seed);
            sec_particle.base_particle.identifier = sec_particle.base_particle.random_number_seed;
            update_trajectory(
                energy_out[secondary_idx],
                angle_out[secondary_idx],
                &mut sec_particle,
            );
            extra.push(MCBaseParticle::new(&sec_particle));
        }
    }

    update_trajectory(energy_out[0], angle_out[0], particle);
    particle.energy_group = mcco
        .nuclear_data
        .get_energy_groups(particle.base_particle.kinetic_energy);

    if n_out > 1 {
        extra.push(MCBaseParticle::new(particle));
    }

    n_out == 1
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::sim::TINY_FLOAT,
        data::{direction_cosine::DirectionCosine, mc_vector::MCVector},
    };
    use num::Float;

    #[test]
    fn trajectory() {
        let mut pp: MCParticle<f64> = MCParticle::default();
        // init data
        let vv: MCVector<f64> = MCVector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let d_cos: DirectionCosine<f64> = DirectionCosine {
            alpha: 1.0 / 3.0.sqrt(),
            beta: 1.0 / 3.0.sqrt(),
            gamma: 1.0 / 3.0.sqrt(),
        };
        let e: f64 = 1.0;
        pp.base_particle.velocity = vv;
        pp.direction_cosine = d_cos;
        pp.base_particle.kinetic_energy = e;
        let mut seed: u64 = 90374384094798327;
        let energy = rng_sample(&mut seed);
        let angle = rng_sample(&mut seed);

        // update & print result
        update_trajectory(energy, angle, &mut pp);

        assert!((pp.direction_cosine.alpha - 0.620283).abs() < 1.0e-6);
        assert!((pp.direction_cosine.beta - 0.620283).abs() < 1.0e-6);
        assert!((pp.direction_cosine.gamma - (-0.480102)).abs() < 1.0e-6);
        assert!((pp.base_particle.kinetic_energy - energy).abs() < TINY_FLOAT);
    }
}
