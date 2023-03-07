use super::mc_location::MCLocation;

pub const N_POINTS_PER_FACET: usize = 3;

/// Enum used to categorize the event a particle
/// undergo when reaching a given facet.
#[derive(Debug, Clone, Copy, Default)]
pub enum MCSubfacetAdjacencyEvent {
    #[default]
    AdjacencyUndefined = 0,
    BoundaryEscape,
    BoundaryReflection,
    TransitOnProcessor,
    TransitOffProcessor,
}

/// Sub-structure for adjacent facet representation.
#[derive(Debug, Default)]
pub struct SubfacetAdjacency {
    pub event: MCSubfacetAdjacencyEvent,
    pub current: MCLocation,
    pub adjacent: MCLocation,
    pub neighbor_index: Option<usize>,
    pub neighbor_global_domain: Option<usize>,
    pub neighbor_foreman: Option<usize>,
}

/// Structure for adjacent facet representation
#[derive(Debug)]
pub struct MCFacetAdjacency {
    pub subfacet: SubfacetAdjacency,
    pub num_points: usize,
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
    pub facet: [MCFacetAdjacency; 24], // need to find the correct way we get 24 to rewrite it with const
    pub point: [usize; 14],            // same with 14
}
