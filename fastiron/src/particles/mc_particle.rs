//! Extended code for particles
//!
//! This module contains code of an extended particle structure used
//! for computations.

use std::iter::zip;

use num::{one, zero, FromPrimitive};
use tinyvec::ArrayVec;

use crate::{
    constants::CustomFloat,
    data::{
        mc_vector::MCVector,
        nuclear_data::{NuclearDataReaction, ReactionType},
        tallies::MCTallyEvent,
    },
    utils::mc_rng_state::{rng_sample, spawn_rn_seed},
};

use super::particle_collection::ParticleCollection;

/// Custom enum used to model a particle's species.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Default)]
pub enum Species {
    /// Invalid value.
    Unknown = -1,
    #[default]
    /// Valid value. Quicksilver only supported one particle type.
    Known = 0, // \o/
}

/// Structure used to hold all data of a particle.
///
/// This is mostly used for computations during the tracking section.
#[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
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
    pub facet_normal: MCVector<T>,
}

impl<T: CustomFloat> MCParticle<T> {
    /// Update the particle's field to model its movement along its
    /// direction, for the segment length.
    pub fn move_particle_along_segment(&mut self) {
        self.coordinate += self.direction * self.segment_path_length;
    }

    /// Update the particle's trajectory with new energy & angle.
    pub fn update_trajectory(&mut self, energy: T, angle: T) {
        // constants
        let pi: T = T::pi();
        let one: T = one();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();

        // value for update
        let cos_theta: T = angle;
        let sin_theta: T = (one - cos_theta * cos_theta).sqrt();
        let rdm_number: T = rng_sample(&mut self.random_number_seed);
        let phi = two * pi * rdm_number;
        let sin_phi: T = phi.sin();
        let cos_phi: T = phi.cos();

        // update
        self.kinetic_energy = energy;
        self.rotate_direction(sin_theta, cos_theta, sin_phi, cos_phi);
        self.sample_num_mfp();
    }

    /// Reflects a particle off a reflection-type boundary.
    ///
    /// This function is called when a particle undergo a reflection event at
    /// the boundary of the problem. Note that the reflection does not result
    /// in a loss of energy.
    pub fn reflect(&mut self) {
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let dot: T = two * self.direction.dot(&self.facet_normal);
        if dot > zero() {
            self.direction -= self.facet_normal * dot;
        }
    }

    /// Computes the particle speed from its energy. Note that this computation
    /// should be species-specific.
    pub fn get_speed(&self) -> T {
        let rest_mass_energy: T = T::neutron_mass_energy();
        let speed_of_light: T = T::light_speed();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        speed_of_light
            * (self.kinetic_energy * (self.kinetic_energy + two * (rest_mass_energy))
                / ((self.kinetic_energy + rest_mass_energy)
                    * (self.kinetic_energy + rest_mass_energy)))
                .sqrt()
    }

    /// Return the starting cross section for reaction sampling.
    pub fn get_current_xs(&mut self) -> T {
        self.total_cross_section * rng_sample::<T>(&mut self.random_number_seed)
    }

    /// Uses a PRNG to sample new energy & angle after a reaction.
    ///
    /// Since reaction type is specified when the method is called, we assume
    /// that the result will be treated correctly by the calling code.
    pub fn sample_collision(
        &mut self,
        reaction: &NuclearDataReaction<T>,
        material_mass: T,
        extra: &mut ParticleCollection<T>,
    ) -> usize {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        // Need to replace with tinyvec types
        match reaction.reaction_type {
            ReactionType::Scatter => {
                let energy = self.kinetic_energy
                    * (one - rng_sample::<T>(&mut self.random_number_seed) * (one / material_mass));
                let angle = rng_sample::<T>(&mut self.random_number_seed) * two - one;
                self.update_trajectory(energy, angle);
                1
            }
            ReactionType::Absorption => 0,
            ReactionType::Fission => {
                let num_particle_out: usize = (reaction.nu_bar
                    + rng_sample(&mut self.random_number_seed))
                .to_usize()
                .unwrap();
                let twenty: T = FromPrimitive::from_f64(20.0).unwrap(); // should be Emax ?
                match num_particle_out {
                    0 => (),
                    1 => {
                        let rand_f = (rng_sample::<T>(&mut self.random_number_seed) + one) / two;
                        let energy = twenty * rand_f * rand_f;
                        let angle = rng_sample::<T>(&mut self.random_number_seed) * two - one;
                        self.update_trajectory(energy, angle);
                    }
                    _ => {
                        assert!(num_particle_out < 5); // this is guaranteed by the way we sample and the nu bar value

                        // for the original particle
                        let rand_f = (rng_sample::<T>(&mut self.random_number_seed) + one) / two;
                        let energy = twenty * rand_f * rand_f;
                        let angle = rng_sample::<T>(&mut self.random_number_seed) * two - one;

                        let mut out = ArrayVec::<[(T, T); 5]>::default();
                        out.extend((1..num_particle_out).map(|_| {
                            let rand_f =
                                (rng_sample::<T>(&mut self.random_number_seed) + one) / two;
                            let energy_out = twenty * rand_f * rand_f;
                            let angle_out =
                                rng_sample::<T>(&mut self.random_number_seed) * two - one;
                            (energy_out, angle_out)
                        }));

                        let mut seeds = ArrayVec::<[u64; 5]>::default();
                        seeds.extend(
                            (1..num_particle_out)
                                .map(|_| spawn_rn_seed::<T>(&mut self.random_number_seed)),
                        );

                        let sec_particles = zip(seeds, out).map(|(seed, (energy, angle))| {
                            let mut sec_particle = self.clone();
                            sec_particle.random_number_seed = seed;
                            sec_particle.identifier = seed;
                            sec_particle.update_trajectory(energy, angle);
                            sec_particle
                        });
                        extra.extend(sec_particles);

                        self.update_trajectory(energy, angle);
                    }
                }
                num_particle_out
            }
        }
    }

    /// Sample the number of mean free paths to a collision.
    pub fn sample_num_mfp(&mut self) {
        self.num_mean_free_paths = -one::<T>() * rng_sample::<T>(&mut self.random_number_seed).ln();
    }

    /// Sample a random direction for the particle to face.
    pub fn sample_isotropic(&mut self) {
        let one: T = one();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let pi: T = T::pi();

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
        let one: T = one();
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

    /// Returns an iterator over particles created from a split of the original one (caller).
    /// This is used in the population control algorithm.
    pub fn under_populated_split(
        &mut self,
        split_rr_factor: T,
    ) -> impl Iterator<Item = MCParticle<T>> + '_ {
        let mut split_factor = split_rr_factor.floor();
        if rng_sample::<T>(&mut self.random_number_seed) > split_rr_factor - split_factor {
            split_factor -= one();
        }
        self.weight /= split_rr_factor;

        let n_split: usize = split_factor.to_usize().unwrap();
        // return an iterating object so we can use .flat_map() method
        (0..n_split).map(|_| {
            let mut split_pp = self.clone();
            split_pp.random_number_seed = spawn_rn_seed::<T>(&mut self.random_number_seed);
            split_pp.identifier = split_pp.random_number_seed;

            split_pp
        })
    }

    /// Play russian-roulette with the particle, returning true if the particle survives,
    /// false otherwise. This function is meant to be used along the [`Vec::retain_mut()`]
    /// method.
    pub fn over_populated_rr(&mut self, split_rr_factor: T) -> bool {
        if rng_sample::<T>(&mut self.random_number_seed) > split_rr_factor {
            // particle dies
            false
        } else {
            // particle survives with increased weight
            self.weight /= split_rr_factor;
            true
        }
    }

    /// Play russian-roulette with particle of low statistical weight, returning true if
    /// the particle survives, false otherwise. This function is meant to be used along
    /// the [`Vec::retain_mut()`] method.
    pub fn low_weight_rr(&mut self, relative_weight_cutoff: T, source_particle_weight: T) -> bool {
        let weight_cutoff = relative_weight_cutoff * source_particle_weight;
        if self.weight <= weight_cutoff {
            if rng_sample::<T>(&mut self.random_number_seed) <= relative_weight_cutoff {
                // particle survives with increased weight
                self.weight /= relative_weight_cutoff;
                true
            } else {
                // particle dies
                false
            }
        } else {
            // particle survives
            true
        }
    }
}

unsafe impl<T: CustomFloat> Send for MCParticle<T> {}

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
    fn trajectory() {
        let mut pp: MCParticle<f64> = MCParticle::default();
        // init data
        let e: f64 = 1.0;
        pp.direction = MCVector {
            x: 1.0 / 3.0.sqrt(),
            y: 1.0 / 3.0.sqrt(),
            z: 1.0 / 3.0.sqrt(),
        };
        pp.kinetic_energy = e;
        let mut seed: u64 = 90374384094798327;
        let energy = rng_sample(&mut seed);
        let angle = rng_sample(&mut seed);

        // update & print result
        pp.update_trajectory(energy, angle);

        assert!((pp.direction.x - 0.620283).abs() < 1.0e-6);
        assert!((pp.direction.y - 0.620283).abs() < 1.0e-6);
        assert!((pp.direction.z - (-0.480102)).abs() < 1.0e-6);
        assert!((pp.kinetic_energy - energy).abs() < f64::tiny_float());
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
