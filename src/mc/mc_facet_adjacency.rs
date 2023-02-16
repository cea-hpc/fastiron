use super::mc_location::MCLocation;

#[derive(Debug)]
pub enum MCSubfacetAdjacencyEvent {
    AdjacencyUndefined,
    BoundaryEscape,
    BoundaryReflection,
    TransitOnProcessor,
    TransitOffProcessor,
}

#[derive(Debug)]
pub struct SubfacetAdjacency {
    event: MCSubfacetAdjacencyEvent,
    current: MCLocation,
    adjacent: MCLocation,
    neighbor_index: usize,
    neighbor_global_domain: usize,
    neighbor_foreman: usize,
}

#[derive(Debug)]
pub struct MCFacetAdjacency {
    subfacet: SubfacetAdjacency,
    num_points: u32,
    point: [u32; 3],
}

#[derive(Debug)]
pub struct MCFacetAdjacencyCell {
    facet: Vec<MCFacetAdjacency>,
    point: Vec<u32>,
}
