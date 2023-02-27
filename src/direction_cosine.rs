use num::{Float, FromPrimitive};

use crate::{physical_constants::PI, mc::mc_rng_state::rng_sample};

#[derive(Debug, Clone, Default)]
pub struct DirectionCosine<T: Float> {
    pub alpha: T,
    pub beta: T,
    pub gamma: T,
}

impl<T: Float + FromPrimitive> DirectionCosine<T> {
    /// Generates a random angle.
    pub fn sample_isotropic(&mut self, seed: &mut u64) {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let pi: T = FromPrimitive::from_f64(PI).unwrap();
        
        // sample gamma
        self.gamma = one - two*rng_sample(seed);

        let sine_gamma = (one - self.gamma*self.gamma).sqrt();
        // sample phi and set the other angles using it
        let phi = pi*(two*rng_sample(seed) - one);

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
    pub fn rotate_3d_vector(&mut self, sine_theta: T, cosine_theta: T, sine_phi: T, cosine_phi: T) {
        todo!()
    }
}
