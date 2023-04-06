//! Code used to model the nearest facet to a given particule

use num::{zero, FromPrimitive};

use crate::constants::{sim::HUGE_FLOAT, CustomFloat};

/// Structure used to represent the nearest facet to a particle,
/// holding relevant data for computation.
#[derive(Debug, Clone, Copy)]
pub struct MCNearestFacet<T: CustomFloat> {
    /// Index of the facet the particle is the closest to.
    pub facet: usize,
    /// Distance between facet and particle.
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
