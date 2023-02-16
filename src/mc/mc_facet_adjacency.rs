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
    event: MCSubfacetAdjacencyEvent,
    current: MCLocation,
    adjacent: MCLocation,
    neighbor_index: usize,
    neighbor_global_domain: usize,
    neighbor_foreman: usize,
}

/// Structure for adjacent facet representation
#[derive(Debug)]
pub struct MCFacetAdjacency {
    subfacet: SubfacetAdjacency,
    num_points: u32,
    point: [u32; 3],
}

/// Structure encompassing all adjacent facet to a cell.
#[derive(Debug)]
pub struct MCFacetAdjacencyCell {
    facet: Vec<MCFacetAdjacency>,
    point: Vec<u32>,
}
