use num::{zero, FromPrimitive};

use crate::{
    constants::{physical::PI, CustomFloat},
    mc::mc_rng_state::rng_sample,
};

#[derive(Debug, Clone, Default)]
pub struct DirectionCosine<T: CustomFloat> {
    pub alpha: T,
    pub beta: T,
    pub gamma: T,
}

impl<T: CustomFloat> DirectionCosine<T> {
    /// Generates a random angle.
    pub fn sample_isotropic(&mut self, seed: &mut u64) {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let pi: T = FromPrimitive::from_f64(PI).unwrap();

        // sample gamma
        self.gamma = one - two * rng_sample(seed);

        let sine_gamma = (one - self.gamma * self.gamma).sqrt();
        // sample phi and set the other angles using it
        let phi = pi * (two * rng_sample(seed) - one);

        self.alpha = sine_gamma * phi.cos();
        self.beta = sine_gamma * phi.sin();
    }

    /// Rotates a 3D vector that is defined by the angles Theta and Phi
    /// in a local coordinate frame about a polar angle and azimuthal angle
    /// described by the direction cosine. Hence, caller passes in
    /// sin_Theta and cos_Theta referenced from the local z-axis and sin_Phi
    /// and cos_Phi referenced from the local x-axis to describe the vector V
    /// to be rotated. The direction cosine describes global theta and phi
    /// angles that the vector V is to be rotated about.
    /// `cos_theta_zero`/`sin_theta_zero` and `cos_phi_zero`/`sin_phi_zero`
    /// model the initial position while the arguments of the method caracterize
    /// the rotation. See [this][1] for explanation on the formula.
    ///
    /// [1]: https://en.wikipedia.org/wiki/Spherical_coordinate_system#Integration_and_differentiation_in_spherical_coordinates
    pub fn rotate_3d_vector(&mut self, sine_theta: T, cosine_theta: T, sine_phi: T, cosine_phi: T) {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();
        let threshold: T = FromPrimitive::from_f64(1e-6).unwrap(); // order of TINY_FLOAT.sqrt()

        let cos_theta_zero = self.gamma;
        let sin_theta_zero = (one - cos_theta_zero * cos_theta_zero).sqrt();

        let (cos_phi_zero, sin_phi_zero): (T, T) = if sin_theta_zero < threshold {
            (one, zero())
        } else {
            (self.alpha / sin_theta_zero, self.beta / sin_theta_zero)
        };

        // compute the rotation
        self.alpha = cos_theta_zero * cos_phi_zero * (sine_theta * cosine_phi)
            - sin_phi_zero * (sine_theta * sine_phi)
            + sin_theta_zero * cos_phi_zero * cosine_theta;

        self.beta = cos_theta_zero * sin_phi_zero * (sine_theta * cosine_phi)
            + cos_phi_zero * (sine_theta * sine_phi)
            + sin_theta_zero * sin_phi_zero * cosine_theta;

        self.gamma =
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
        let mut dd: DirectionCosine<f64> = DirectionCosine {
            alpha: 0.2140,
            beta: 0.8621,
            gamma: 0.7821,
        };
        let mut seed: u64 = 90374384094798327;
        dd.sample_isotropic(&mut seed);

        assert_eq!(dd.alpha, 0.9083218129645693);
        assert_eq!(dd.beta, -0.3658911896631176);
        assert_eq!(dd.gamma, 0.2026699815455325);
    }

    #[test]
    fn rotate_vector() {
        let mut dd: DirectionCosine<f64> = DirectionCosine {
            alpha: 0.2140,
            beta: 0.8621,
            gamma: 0.7821,
        };
        dd.rotate_3d_vector(1.0.sin(), 1.0.cos(), 2.0.sin(), 2.0.cos());

        assert_eq!(dd.alpha, -1.0369691350703922);
        assert_eq!(dd.beta, 0.3496694784021821);
        assert_eq!(dd.gamma, 0.6407833194623658);
    }
}
