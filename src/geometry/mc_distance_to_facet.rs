use crate::constants::CustomFloat;

/// Structure used to represent the distance to a given facet.
#[derive(Debug, Clone, Copy, Default)]
pub struct MCDistanceToFacet<T: CustomFloat> {
    pub distance: T,
    pub facet: usize,
    pub subfacet: usize,
}
