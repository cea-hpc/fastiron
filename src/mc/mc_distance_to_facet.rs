use num::{zero, Float};

/// Structure used to represent the distance to a given facet.
#[derive(Debug, Clone, Copy)]
pub struct MCDistanceToFacet<T: Float> {
    pub distance: T,
    pub facet: usize,
    pub subfacet: usize,
}

impl<T: Float> Default for MCDistanceToFacet<T> {
    fn default() -> Self {
        Self {
            distance: zero(),
            facet: 0,
            subfacet: 0,
        }
    }
}
