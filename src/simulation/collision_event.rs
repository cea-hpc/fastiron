//! Event-specific code for particles colliding
//!
//! This module contains code that process particles undergoing a collision
//! from beginning to end. Note that _collision_ refers to reaction with the
//! particle's environment, not in-between particles.

use num::{zero, FromPrimitive};

use crate::{
    constants::{physical::PI, CustomFloat},
    data::{nuclear_data::ReactionType, tallies::Balance},
    montecarlo::MonteCarloData,
    particles::mc_particle::MCParticle,
    simulation::macro_cross_section::macroscopic_cross_section,
    utils::mc_rng_state::{rng_sample, spawn_rn_seed},
};

fn update_trajectory<T: CustomFloat>(energy: T, angle: T, particle: &mut MCParticle<T>) {
    // constants
    let pi: T = FromPrimitive::from_f64(PI).unwrap();
    let one: T = FromPrimitive::from_f64(1.0).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();

    // value for update
    let cos_theta: T = angle;
    let sin_theta: T = (one - cos_theta * cos_theta).sqrt();
    let mut rdm_number: T = rng_sample(&mut particle.random_number_seed);
    let phi = two * pi * rdm_number;
    let sin_phi: T = phi.sin();
    let cos_phi: T = phi.cos();

    // update
    particle.kinetic_energy = energy;
    particle
        .direction_cosine
        .rotate_3d_vector(sin_theta, cos_theta, sin_phi, cos_phi);
    rdm_number = rng_sample(&mut particle.random_number_seed);
    particle.num_mean_free_paths = -one * rdm_number.ln();
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
    mcdata: &MonteCarloData<T>,
    mat_gid: usize,
    cell_nb_density: T,
    particle: &mut MCParticle<T>,
    extra: &mut Vec<MCParticle<T>>,
    balance: &mut Balance,
) -> bool {
    // ==========================
    // Pick an isotope & reaction

    let rdm_number: T = rng_sample(&mut particle.random_number_seed);
    let total_xsection: T = particle.total_cross_section;

    let mut current_xsection: T = total_xsection * rdm_number;

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

    // ================
    // Do the collision

    let mat_mass = mcdata.material_database.mat[mat_gid].mass;
    let (energy_out, angle_out) = mcdata.nuclear_data.isotopes[selected_unique_n][0].reactions
        [selected_react]
        .sample_collision(
            particle.kinetic_energy,
            mat_mass,
            &mut particle.random_number_seed,
        );
    // number of particles resulting from the collision, including the original
    // e.g. zero means the original particle was absorbed or invalidated in some way
    let n_out = energy_out.len();

    // ===================
    // Tally the collision

    balance.collision += 1; // atomic in original code
    match mcdata.nuclear_data.isotopes[selected_unique_n][0].reactions[selected_react].reaction_type
    {
        ReactionType::Scatter => balance.scatter += 1,
        ReactionType::Absorption => balance.absorb += 1,
        ReactionType::Fission => {
            balance.fission += 1;
            balance.produce += n_out as u64;
        }
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
            sec_particle.random_number_seed = spawn_rn_seed::<T>(&mut particle.random_number_seed);
            sec_particle.identifier = sec_particle.random_number_seed;
            update_trajectory(
                energy_out[secondary_idx],
                angle_out[secondary_idx],
                &mut sec_particle,
            );
            extra.push(sec_particle);
        }
    }

    update_trajectory(energy_out[0], angle_out[0], particle);
    particle.energy_group = mcdata
        .nuclear_data
        .get_energy_groups(particle.kinetic_energy);

    if n_out > 1 {
        extra.push(particle.clone());
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
        let d_cos: DirectionCosine<f64> = DirectionCosine {
            dir: MCVector {
                x: 1.0 / 3.0.sqrt(),
                y: 1.0 / 3.0.sqrt(),
                z: 1.0 / 3.0.sqrt(),
            },
        };
        let e: f64 = 1.0;
        pp.direction_cosine = d_cos;
        pp.kinetic_energy = e;
        let mut seed: u64 = 90374384094798327;
        let energy = rng_sample(&mut seed);
        let angle = rng_sample(&mut seed);

        // update & print result
        update_trajectory(energy, angle, &mut pp);

        assert!((pp.direction_cosine.dir.x - 0.620283).abs() < 1.0e-6);
        assert!((pp.direction_cosine.dir.y - 0.620283).abs() < 1.0e-6);
        assert!((pp.direction_cosine.dir.z - (-0.480102)).abs() < 1.0e-6);
        assert!((pp.kinetic_energy - energy).abs() < TINY_FLOAT);
    }
}
