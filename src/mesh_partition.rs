use std::collections::HashMap;

use num::Float;

use crate::{global_fcc_grid::GlobalFccGrid, mc::mc_vector::MCVector};

type MapType = HashMap<u64, CellInfo>;

/// Structure used to hold cell information.
#[derive(Debug)]
pub struct CellInfo {
    /// Domain global identifier.
    pub domain_gid: usize,
    /// ?
    pub foreman: usize, // ?
    /// Domain index?
    pub domain_index: usize,
    /// Cell index.
    pub cell_index: usize,
}

impl Default for CellInfo {
    fn default() -> Self {
        todo!()
    }
}

/// Structure used to represent the mesh partition of the space.
/// Holds the different cells' information.
#[derive(Debug)]
pub struct MeshPartition {
    /// Domain global identifier.
    pub domain_gid: usize,
    /// Domain index?
    pub domain_index: usize,
    /// ?
    pub foreman: usize,
    /// Map linking cell global identifier to thair [CellInfo] structure
    pub cell_info_map: MapType,
    /// ?
    pub nbr_domains: Vec<usize>,
}

impl MeshPartition {
    /// Constructor. The structure is NOT ready to be used directly.
    pub fn new(domain_gid: usize, domain_index: usize, foreman: usize) -> Self {
        todo!()
    }
    /// Builds the mesh partition.
    pub fn build_mesh_partition<T: Float>(
        &mut self,
        grid: &GlobalFccGrid<T>,
        centers: Vec<MCVector<T>>,
    ) {
        todo!()
    }
}

fn assign_cells_to_domain() {}

fn build_cell_idx_map() {}

fn add_nbrs_to_flood() {}
