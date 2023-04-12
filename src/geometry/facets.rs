//! Facet modelling & related computation

use crate::constants::CustomFloat;

/// Structure used to represent the distance to a given facet.
///
/// This structure is used in order to group information used when assessing the
/// next event for a particle and, in the case of a facet crossing, going through
/// with it.
#[derive(Debug, Clone, Copy, Default)]
pub struct MCDistanceToFacet<T: CustomFloat> {
    /// Distance to the given facet in cm.
    pub distance: T,
    /// Index of the given facet.
    pub facet: usize,
    /// Index of the given sub-facet.
    pub subfacet: usize,
}
