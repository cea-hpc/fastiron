use num::Float;

use crate::{
    bulk_storage::BulkStorage, decomposition_object::DecompositionObject,
    global_fcc_grid::GlobalFccGrid, material_database::MaterialDatabase,
    mesh_partition::MeshPartition, parameters::Parameters,
};

use super::{
    mc_cell_state::MCCellState,
    mc_facet_adjacency::{MCFacetAdjacency, MCSubfacetAdjacencyEvent},
    mc_facet_geometry::{MCFacetGeometryCell, MCGeneralPlane},
    mc_vector::MCVector,
};

/// Structure that manages a data set on a mesh_like geometry
#[derive(Debug)]
pub struct MCMeshDomain<T: Float> {
    pub domain_gid: usize,

    pub nbr_domain_gid: Vec<usize>, // maybe not usize
    pub nbr_rank: Vec<usize>,       // maybe not usize

    pub node: Vec<MCVector<T>>,
    pub cell_connectivity: Vec<MCFacetAdjacency>,

    pub cell_geometry: Vec<MCFacetGeometryCell<T>>,
    pub connectivity_facet_storage: BulkStorage<MCFacetAdjacency>,
    pub connectivity_point_storage: BulkStorage<i32>,
    pub geom_facet_storage: BulkStorage<MCGeneralPlane<T>>,
}

impl<T: Float> MCMeshDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition<T>,
        grid: &GlobalFccGrid,
        ddc: &DecompositionObject,
        boundary_condition: &[MCSubfacetAdjacencyEvent],
    ) -> Self {
        todo!()
    }
}

/// Structure used to manage a region on a domain
#[derive(Debug)]
pub struct MCDomain<T: Float> {
    // pub domain_index: usize, // unused?
    pub global_domain: usize,
    pub cell_state: Vec<MCCellState<T>>,
    pub cached_cross_section_storage: BulkStorage<f64>,
    pub mesh: MCMeshDomain<T>,
}

impl<T: Float> MCDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition<T>,
        grid: &GlobalFccGrid,
        ddc: &DecompositionObject,
        params: &Parameters,
        material_database: &MaterialDatabase<T>,
        num_energy_groups: u32, // maybe usize?
    ) -> Self {
        todo!()
    }

    pub fn clear_cross_section_cache(num_energy_groups: u32) {}
}
