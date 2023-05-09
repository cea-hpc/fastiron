//! Code used to asociate a location to a particle
//!
//! **May be moved to another file**.

/// Structure used to model a location in the problem.
///
/// This is associated to a particle; It gives the domain and cell it belongs
/// to as well as the associated facet.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MCLocation {
    /// Index of the domain the particle belongs to.
    pub domain: Option<usize>,
    /// Index of the cell the particle belongs to.
    pub cell: Option<usize>,
    /// Index of the facet associated to the particle. A value of `None` here
    /// can be expected depending on the satge of the tracking algorithm.
    pub facet: Option<usize>,
}
