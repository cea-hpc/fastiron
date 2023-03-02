use num::Float;

use crate::{
    bulk_storage::BulkStorage, decomposition_object::DecompositionObject,
    global_fcc_grid::GlobalFccGrid, material_database::MaterialDatabase,
    mesh_partition::MeshPartition, parameters::Parameters,
};

use super::{
    mc_cell_state::MCCellState,
    mc_facet_adjacency::{MCFacetAdjacency, MCSubfacetAdjacencyEvent, MCFacetAdjacencyCell},
    mc_facet_geometry::{MCFacetGeometryCell, MCGeneralPlane},
    mc_vector::MCVector,
};

/// Structure that manages a data set on a mesh-like geometry
#[derive(Debug)]
pub struct MCMeshDomain<T: Float> {
    /// Global identifier of the domain
    pub domain_gid: usize,
    /// List of domain global identifiers
    pub nbr_domain_gid: Vec<usize>, // maybe not usize
    /// List of ranks
    pub nbr_rank: Vec<usize>, // maybe not usize
    /// List of nodes
    pub node: Vec<MCVector<T>>,
    /// List of connectivity between cells
    pub cell_connectivity: Vec<MCFacetAdjacencyCell>,
    /// List of cells
    pub cell_geometry: Vec<MCFacetGeometryCell<T>>,

    /// Needs replacement
    pub connectivity_facet_storage: BulkStorage<MCFacetAdjacency>,
    /// Needs replacement
    pub connectivity_point_storage: BulkStorage<i32>,
    /// Needs replacement
    pub geom_facet_storage: BulkStorage<MCGeneralPlane<T>>,
}

impl<T: Float> MCMeshDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition<T>,
        grid: &GlobalFccGrid<T>,
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
    /// Global domain number
    pub global_domain: usize,
    /// List of cells and their state (See [MCCellState] for more)
    pub cell_state: Vec<MCCellState<T>>,
    /// Needs replacement
    pub cached_cross_section_storage: BulkStorage<f64>,
    /// Mesh of the domain
    pub mesh: MCMeshDomain<T>,
}

impl<T: Float> MCDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition<T>,
        grid: &GlobalFccGrid<T>,
        ddc: &DecompositionObject,
        params: &Parameters,
        material_database: &MaterialDatabase<T>,
        num_energy_groups: u32, // maybe usize?
    ) -> Self {
        todo!()
    }

    /// Clears the cross section cache for future uses.
    pub fn clear_cross_section_cache(num_energy_groups: u32) {}
}
