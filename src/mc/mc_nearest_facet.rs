use num::Float;

/// Structure used to represent the nearest facet to a point,
/// holding relevant data for computation.
#[derive(Debug, Clone, Copy)]
pub struct MCNearestFacet<T: Float> {
    /// Facet the particle is the closest to
    pub facet: usize,
    /// Distance between facet and particle
    pub distance_to_facet: T,
    /// Dot product between facet and speed/position vector?
    pub dot_product: T,
}

impl<T: Float> Default for MCNearestFacet<T> {
    fn default() -> Self {
        todo!()
    }
}
