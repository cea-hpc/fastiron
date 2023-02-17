use num::Float;

#[derive(Debug, Clone)]
pub struct DirectionCosine<T: Float> {
    pub alpha: T,
    pub beta: T,
    pub gamma: T,
}
