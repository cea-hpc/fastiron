//! Code related adjacency-modelling in the mesh.
//!
//! This module contains code related to the modelling of adjacency between cells
//! through the use of facet-specific data.

use crate::constants::mesh::{N_FACETS_OUT, N_POINTS_INTERSEC, N_POINTS_PER_FACET};

use super::mc_location::MCLocation;

/// Enum used to categorize the event a particle
/// undergo when reaching a given facet.
///
/// This value essentially depends on the nature of what is on the other side.
#[derive(Debug, Clone, Copy, Default)]
pub enum MCSubfacetAdjacencyEvent {
    /// Default value. This will generate error at runtime if not initialized
    /// correctly.
    #[default]
    AdjacencyUndefined = 0,
    /// Value correspnding to an escape event. The facet is located at the
    /// edge of the problem and the behavior is set to allow escape.
    BoundaryEscape,
    /// Value correspnding to an reflection event. The facet is located at the
    /// edge of the problem and the behavior of the facet is set to relfect.
    BoundaryReflection,
    /// Value corresponding to an intra-problem crossing event. The neighboring
    /// facet belongs to a cell managed by the same processor.
    TransitOnProcessor,
    /// Value corresponding to an intra-problem crossing event. The neighboring
    /// facet belongs to a cell managed by a different processors.
    TransitOffProcessor,
}

/// Sub-structure for adjacent facet representation.
///
/// This structure is _oriented_, i.e. there is a current cell and a neighbor
/// cell.
#[derive(Debug, Default)]
pub struct SubfacetAdjacency {
    /// Event associated with the facet junction.
    pub event: MCSubfacetAdjacencyEvent,
    /// Current location.
    pub current: MCLocation,
    /// Neighboring location.
    pub adjacent: MCLocation,
    /// Neighbor index.
    pub neighbor_index: Option<usize>,
    /// Neighbor domain global identifier.
    pub neighbor_global_domain: Option<usize>,
    /// Neighbor foreman.
    pub neighbor_foreman: Option<usize>,
}

/// Structure for adjacent facet representation.
#[derive(Debug)]
pub struct MCFacetAdjacency {
    /// Adjacency data.
    pub subfacet: SubfacetAdjacency,
    /// Number of points of the facet. See [N_POINTS_PER_FACET] for more
    /// information. **Since the point list is a static array, this may be
    /// removed**.
    pub num_points: usize,
    /// Point indexes for this facet. The points are defined in a private constant
    /// in the [mc_domain][super::mc_domain] module.
    pub point: [Option<usize>; N_POINTS_PER_FACET],
}

impl Default for MCFacetAdjacency {
    fn default() -> Self {
        Self {
            subfacet: Default::default(),
            num_points: N_POINTS_PER_FACET,
            point: [None; N_POINTS_PER_FACET],
        }
    }
}

/// Structure encompassing all adjacent facet to a cell.
#[derive(Debug, Default)]
pub struct MCFacetAdjacencyCell {
    pub facet: [MCFacetAdjacency; N_FACETS_OUT],
    pub point: [usize; N_POINTS_INTERSEC],
}
