use num::FromPrimitive;

use crate::{
    constants::{
        physical::{LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY, PI},
        CustomFloat,
    },
    macro_cross_section::macroscopic_cross_section,
    mc::{
        mc_particle::MCParticle,
        mc_rng_state::{rng_sample, spawn_rn_seed},
    },
    montecarlo::MonteCarlo,
    nuclear_data::ReactionType,
};

/// Update the a particle's energy and trajectory after a collision.
pub fn update_trajectory<T: CustomFloat>(energy: T, angle: T, particle: &mut MCParticle<T>) {
    // constants
    let pi: T = FromPrimitive::from_f64(PI).unwrap();
    let c: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
    let nrm: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
    let one: T = FromPrimitive::from_f64(1.0).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();

    // value for update
    let cos_theta: T = angle;
    let sin_theta: T = (one - cos_theta * cos_theta).sqrt();
    let mut rdm_number: T = rng_sample(&mut particle.random_number_seed);
    let phi = two * pi * rdm_number;
    let sin_phi: T = phi.sin();
    let cos_phi: T = phi.cos();
    let speed: T = c * (one - nrm * nrm / ((energy + nrm) * (energy + nrm))).sqrt();

    // update
    particle.kinetic_energy = energy;
    particle
        .direction_cosine
        .rotate_3d_vector(sin_theta, cos_theta, sin_phi, cos_phi);
    particle.velocity.x = speed * particle.direction_cosine.alpha;
    particle.velocity.y = speed * particle.direction_cosine.beta;
    particle.velocity.z = speed * particle.direction_cosine.gamma;
    rdm_number = rng_sample(&mut particle.random_number_seed);
    particle.num_mean_free_paths = -one * rdm_number.ln();
}

/// Computes and transform accordingly a [MCParticle] object that
/// undergo a collision. Returns true if the particle will continue
pub fn collision_event<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    mc_particle: &mut MCParticle<T>,
    tally_idx: usize,
) -> bool {
    let mat_gidx = mcco.domain[mc_particle.domain].cell_state[mc_particle.cell].material;

    // ==========================
    // Pick an isotope & reaction
    let rdm_number: T = rng_sample(&mut mc_particle.random_number_seed);
    let total_xsection: T = mc_particle.total_cross_section;
    println!("total xs: {total_xsection}");

    let mut current_xsection: T = total_xsection * rdm_number;
    println!("starting xs: {current_xsection}");

    let mut selected_iso: usize = usize::MAX; // sort of a magic value
    let mut selected_unique_n: usize = usize::MAX;
    let mut selected_react: usize = usize::MAX;

    let n_iso: usize = mcco.material_database.mat[mat_gidx].iso.len();

    loop {
        //println!("infinite loop? current xs: {current_xsection}");
        for iso_idx in 0..n_iso {
            let unique_n: usize = mcco.material_database.mat[mat_gidx].iso[iso_idx].gid;
            let n_reactions: usize = mcco.nuclear_data.get_number_reactions(unique_n);
            for reaction_idx in 0..n_reactions {
                current_xsection -= macroscopic_cross_section(
                    mcco,
                    reaction_idx,
                    mc_particle.domain,
                    mc_particle.cell,
                    iso_idx,
                    mc_particle.energy_group,
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
            mc_particle.kinetic_energy,
            mat_mass,
            &mut mc_particle.random_number_seed,
        );

    // ===================
    // Tally the collision
    mcco.tallies.balance_task[tally_idx].collision += 1; // atomic in original code
    match mcco.nuclear_data.isotopes[selected_unique_n][0].reactions[selected_react].reaction_type {
        ReactionType::Scatter => (),
        ReactionType::Absorption => (),
        ReactionType::Fission => (),
        ReactionType::Undefined => panic!(),
    }

    let n_out = energy_out.len();
    // Early return
    if n_out == 0 {
        return false;
    }

    if n_out > 1 {
        for secondary_idx in 1..energy_out.len() {
            let mut sec_particle = mc_particle.clone();
            sec_particle.random_number_seed =
                spawn_rn_seed::<T>(&mut mc_particle.random_number_seed);
            sec_particle.identifier = sec_particle.random_number_seed;
            update_trajectory(
                energy_out[secondary_idx],
                angle_out[secondary_idx],
                &mut sec_particle,
            );
            mcco.particle_vault_container
                .add_extra_particle(sec_particle);
        }
    }

    update_trajectory(energy_out[0], angle_out[0], mc_particle);
    mc_particle.energy_group = mcco
        .nuclear_data
        .get_energy_groups(mc_particle.kinetic_energy);

    if n_out > 1 {
        mcco.particle_vault_container
            .add_extra_particle(mc_particle.clone());
    }

    n_out == 1
}
