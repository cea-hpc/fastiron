use super::mc_location::MCLocation;

/// Enum used to categorize the event a particle
/// undergo when reaching a given facet.
#[derive(Debug)]
pub enum MCSubfacetAdjacencyEvent {
    AdjacencyUndefined,
    BoundaryEscape,
    BoundaryReflection,
    TransitOnProcessor,
    TransitOffProcessor,
}

/// Sub-structure for adjacent facet representation.
#[derive(Debug)]
pub struct SubfacetAdjacency {
    pub event: MCSubfacetAdjacencyEvent,
    pub current: MCLocation,
    pub adjacent: MCLocation,
    pub neighbor_index: usize,
    pub neighbor_global_domain: usize,
    pub neighbor_foreman: usize,
}

/// Structure for adjacent facet representation
#[derive(Debug)]
pub struct MCFacetAdjacency {
    pub subfacet: SubfacetAdjacency,
    pub num_points: u32,
    pub point: [u32; 3],
}

/// Structure encompassing all adjacent facet to a cell.
#[derive(Debug)]
pub struct MCFacetAdjacencyCell {
    pub facet: Vec<MCFacetAdjacency>,
    pub point: Vec<u32>,
}
