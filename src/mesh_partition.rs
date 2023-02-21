use std::{collections::HashMap, marker::PhantomData};

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
pub struct MeshPartition<T: Float> {
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
    float_type: PhantomData<T>,
}

impl<T: Float> MeshPartition<T> {
    /// Constructor. The structure is NOT ready to be used directly.
    pub fn new(domain_gid: usize, domain_index: usize, foreman: usize) -> Self {
        todo!()
    }
    /// Builds the mesh partition.
    pub fn build_mesh_partition(&mut self, grid: &GlobalFccGrid<T>, centers: Vec<MCVector<T>>) {
        todo!()
    }
}
