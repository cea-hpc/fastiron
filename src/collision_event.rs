use num::{Float, FromPrimitive};

use crate::{mc::{mc_particle::MCParticle, mc_rng_state::rng_sample}, montecarlo::MonteCarlo, physical_constants::{PI, LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY}};

/// Update the a particle's energy and trajectory after a collision.
pub fn update_trajectory<T: Float + FromPrimitive>(energy: T, angle: T, particle: &mut MCParticle<T>) {
    // constants
    let pi: T = FromPrimitive::from_f64(PI).unwrap();
    let c: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
    let nrm: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
    let one: T = FromPrimitive::from_f64(1.0).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();

    // value for update
    let cos_theta: T = angle;
    let sin_theta: T = (one - cos_theta*cos_theta).sqrt();
    let mut rdm_number: T = rng_sample(&mut particle.random_number_seed);
    let phi = two * pi * rdm_number;
    let sin_phi: T = phi.sin();
    let cos_phi: T = phi.cos();
    let speed: T = c * (one - nrm*nrm/((energy+nrm)*(energy+nrm))).sqrt();

    // update
    particle.kinetic_energy = energy;
    particle.direction_cosine.rotate_3d_vector(sin_theta, cos_theta, sin_phi, cos_phi);
    particle.velocity.x = speed * particle.direction_cosine.alpha;
    particle.velocity.y = speed * particle.direction_cosine.beta;
    particle.velocity.z = speed * particle.direction_cosine.gamma;
    rdm_number = rng_sample(&mut particle.random_number_seed);
    particle.num_mean_free_paths = -one * rdm_number.ln();
}

/// Computes and transform accordingly a [MCParticle] object that
/// undergo a collision.
pub fn collision_event<T: Float>(
    mcco: &mut MonteCarlo<T>,
    mc_particle: &MCParticle<T>,
    tally_idx: usize,
) -> bool {
    todo!()
}