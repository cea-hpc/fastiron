use num::Float;

#[derive(Debug, Clone)]
pub struct DirectionCosine<T: Float> {
    pub alpha: T,
    pub beta: T,
    pub gamma: T,
}

impl<T: Float> DirectionCosine<T> {
    /// Generates a random angle.
    pub fn sample_isotropic(&mut self, seed: &u64) {
        todo!()
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

impl<T: Float> Default for DirectionCosine<T> {
    fn default() -> Self {
        todo!()
    }
}
