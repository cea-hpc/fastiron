use num::Float;

/// Custom type for vector representation.
#[derive(Debug)]
pub struct MCVector<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}
