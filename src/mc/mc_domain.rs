use crate::bulk_storage::BulkStorage;

use super::{
    mc_cell_state::MCCellState, mc_facet_adjacency::MCFacetAdjacency,
    mc_facet_geometry::{MCFacetGeometryCell, MCGeneralPlane}, mc_vector::MCVector,
};

/// Structure that manages a data set on a mesh_like geometry
#[derive(Debug)]
pub struct MCMeshDomain {
    pub domain_gid: usize,

    pub nbr_domain_gid: Vec<usize>, // maybe not usize
    pub nbr_rank: Vec<usize>,       // maybe not usize

    pub node: Vec<MCVector>,
    pub cell_connectivity: Vec<MCFacetAdjacency>,

    pub cell_geometry: Vec<MCFacetGeometryCell>,
    pub connectivity_facet_storage: BulkStorage<MCFacetAdjacency>,
    pub connectivity_point_storage: BulkStorage<i32>,
    pub geom_facet_storage: BulkStorage<MCGeneralPlane>,
}

/// Structure used to manage a region on a domain
#[derive(Debug)]
pub struct MCDomain {
    // pub domain_index: usize, // unused?
    pub global_domain: usize,
    pub cell_state: Vec<MCCellState>,
    // pub cached_cross_section_storage: BulkStorage<f64>, ??
    pub mesh: MCMeshDomain,
}
