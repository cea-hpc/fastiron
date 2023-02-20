use num::Float;

#[derive(Debug, Clone)]
pub struct DirectionCosine<T: Float> {
    pub alpha: T,
    pub beta: T,
    pub gamma: T,
}

impl<T: Float> DirectionCosine<T> {
    pub fn sample_isotropic(&mut self, seed: &u64) {
        todo!()
    }

    pub fn rotate_3d_vector(&mut self, sine_theta: T, cosine_theta: T, sine_phi: T, cosine_phi: T) {
        todo!()
    }
}

impl<T: Float> Default for DirectionCosine<T> {
    fn default() -> Self {
        todo!()
    }
}
