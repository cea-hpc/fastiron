use num::{zero, FromPrimitive};

use crate::constants::{physical::HUGE_FLOAT, CustomFloat};

/// Structure used to represent the nearest facet to a point,
/// holding relevant data for computation.
#[derive(Debug, Clone, Copy)]
pub struct MCNearestFacet<T: CustomFloat> {
    /// Facet the particle is the closest to
    pub facet: usize,
    /// Distance between facet and particle
    pub distance_to_facet: T,
    /// Dot product between facet and direction vector.
    pub dot_product: T,
}

impl<T: CustomFloat> Default for MCNearestFacet<T> {
    fn default() -> Self {
        Self {
            facet: 0,
            distance_to_facet: FromPrimitive::from_f64(HUGE_FLOAT).unwrap(),
            dot_product: zero(),
        }
    }
}
