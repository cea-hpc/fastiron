//! Extended code for particles
//!
//! This module contains code of an extended particle structure used
//! for computations.

use num::{zero, FromPrimitive};

use crate::{
    constants::{
        physical::{LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY, PI},
        CustomFloat,
    },
    data::{mc_vector::MCVector, tallies::MCTallyEvent},
    utils::mc_rng_state::rng_sample,
};

/// Custom enum used to model a particle's species.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Species {
    /// Invalid value.
    Unknown = -1,
    #[default]
    /// Valid value. Quicksilver only supportedone particle type.
    Known = 0, // \o/
}

/// Structure used to hold all data of a particle.
///
/// This is mostly used for computations during the tracking section.
#[derive(Debug, Default, Clone)]
pub struct MCParticle<T: CustomFloat> {
    /// Current position.
    pub coordinate: MCVector<T>,
    /// Direction of the particle as a normalized `(x, y, z)` vector.
    pub direction: MCVector<T>,
    /// Kinetic energy.
    pub kinetic_energy: T,
    /// Current energy group the particle belong to.
    pub energy_group: usize,
    /// Weight.
    pub weight: T,
    /// Species of the particle.
    pub species: Species,

    /// Time remaining before this particle hit census.
    pub time_to_census: T,
    /// Age.
    pub age: T,

    /// Cache-ing the current total cross section/
    pub total_cross_section: T,
    /// Distance to a collision.
    pub mean_free_path: T,
    /// Distance this particle travels in a segment.
    pub segment_path_length: T,
    /// Number of mean free paths to a collision.
    pub num_mean_free_paths: T,
    /// Number of segments the particle travelled.
    pub num_segments: T,

    /// Random number seed for the rng for this particle.
    pub random_number_seed: u64,
    /// Unique ID used to identify and track individual particles.
    pub identifier: u64,
    /// Last event this particle underwent.
    pub last_event: MCTallyEvent,

    /// Current domain in the spatial grid.
    pub domain: usize,
    /// Current cell in the current domain.
    pub cell: usize,
    /// Nearest facet.
    pub facet: usize,
    /// Normal dot product value kept when crossing a facet.
    pub normal_dot: T,
}

impl<T: CustomFloat> MCParticle<T> {
    /// Update the particle's field to model its movement along the specified
    /// direction and distance
    pub fn move_particle_along_segment(&mut self) {
        self.coordinate += self.direction * self.segment_path_length;
    }

    pub fn get_speed(&self) -> T {
        let rest_mass_energy: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
        let speed_of_light: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        speed_of_light
            * (self.kinetic_energy * (self.kinetic_energy + two * (rest_mass_energy))
                / ((self.kinetic_energy + rest_mass_energy)
                    * (self.kinetic_energy + rest_mass_energy)))
                .sqrt()
    }

    /// Sample a random direction for the particle to face.
    pub fn sample_isotropic(&mut self) {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let pi: T = FromPrimitive::from_f64(PI).unwrap();

        // sample gamma
        self.direction.z = one - two * rng_sample(&mut self.random_number_seed);
        let sine_gamma = (one - self.direction.z * self.direction.z).sqrt();

        // sample phi and set the other angles using it
        let phi = pi * (two * rng_sample(&mut self.random_number_seed) - one);

        self.direction.x = sine_gamma * phi.cos();
        self.direction.y = sine_gamma * phi.sin();
    }

    /// Rotates a 3D vector that is defined by the angles Theta and Phi
    /// in a local coordinate frame about a polar angle and azimuthal angle
    /// described by the direction cosine. Hence, caller passes in
    /// `sin_Theta` and `cos_Theta` referenced from the local z-axis and `sin_Phi`
    /// and `cos_Phi` referenced from the local x-axis to describe the vector V
    /// to be rotated. The direction cosine describes global theta and phi
    /// angles that the vector V is to be rotated about.
    /// `cos_theta_zero`/`sin_theta_zero` and `cos_phi_zero`/`sin_phi_zero`
    /// model the initial position while the arguments of the method caracterize
    /// the rotation. See [this][1] for explanation on the formula.
    ///
    /// [1]: https://en.wikipedia.org/wiki/Spherical_coordinate_system#Integration_and_differentiation_in_spherical_coordinates
    pub fn rotate_direction(&mut self, sine_theta: T, cosine_theta: T, sine_phi: T, cosine_phi: T) {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();
        let threshold: T = FromPrimitive::from_f64(1e-6).unwrap(); // order of TINY_FLOAT.sqrt()

        let cos_theta_zero = self.direction.z;
        let sin_theta_zero = (one - cos_theta_zero * cos_theta_zero).sqrt();

        let (cos_phi_zero, sin_phi_zero): (T, T) = if sin_theta_zero < threshold {
            (one, zero())
        } else {
            (
                self.direction.x / sin_theta_zero,
                self.direction.y / sin_theta_zero,
            )
        };

        // compute the rotation
        self.direction.x = cos_theta_zero * cos_phi_zero * (sine_theta * cosine_phi)
            - sin_phi_zero * (sine_theta * sine_phi)
            + sin_theta_zero * cos_phi_zero * cosine_theta;

        self.direction.y = cos_theta_zero * sin_phi_zero * (sine_theta * cosine_phi)
            + cos_phi_zero * (sine_theta * sine_phi)
            + sin_theta_zero * sin_phi_zero * cosine_theta;

        self.direction.z =
            -sin_theta_zero * (sine_theta * cosine_phi) + zero() + cos_theta_zero * cosine_theta;
    }
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    use super::*;
    use num::Float;

    #[test]
    fn sample_isotropic() {
        let mut particle: MCParticle<f64> = MCParticle {
            random_number_seed: 90374384094798327,
            ..Default::default()
        };

        particle.sample_isotropic();

        assert_eq!(particle.direction.x, 0.9083218129645693);
        assert_eq!(particle.direction.y, -0.3658911896631176);
        assert_eq!(particle.direction.z, 0.2026699815455325);
    }

    #[test]
    fn rotate_direction() {
        let alpha = 0.2140;
        let beta = 0.8621;
        let gamma = 0.7821;
        let mut particle: MCParticle<f64> = MCParticle {
            direction: MCVector {
                x: alpha,
                y: beta,
                z: gamma,
            },
            ..Default::default()
        };
        particle.rotate_direction(1.0.sin(), 1.0.cos(), 2.0.sin(), 2.0.cos());

        assert_eq!(particle.direction.x, -1.0369691350703922);
        assert_eq!(particle.direction.y, 0.3496694784021821);
        assert_eq!(particle.direction.z, 0.6407833194623658);
    }
}
