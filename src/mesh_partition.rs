use std::collections::HashMap;

use num::Float;

use crate::{global_fcc_grid::{GlobalFccGrid, Tuple}, mc::mc_vector::MCVector};

type MapType = HashMap<usize, CellInfo>;

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
    /// Change so that it returns directly a complete object? Just call build in the constructor?
    pub fn new(domain_gid: usize, domain_index: usize, foreman: usize) -> Self {
        Self { domain_gid, domain_index, foreman, cell_info_map: Default::default(), nbr_domains: Default::default() }
    }
    /// Builds the mesh partition.
    pub fn build_mesh_partition<T: Float>(
        &mut self,
        grid: &GlobalFccGrid<T>,
        centers: &[MCVector<T>],
    ) {
        self.assign_cells_to_domain(centers, grid);

        self.build_cell_idx_map(grid);
    }

    fn assign_cells_to_domain<T: Float>(&mut self, domain_center: &[MCVector<T>], grid: &GlobalFccGrid<T>) {}

    fn build_cell_idx_map<T: Float>(&mut self, grid: &GlobalFccGrid<T>) {
        let mut n_local_cells: usize = 0;
        // init a map
        let mut remote_domain_map: HashMap<usize, usize> = Default::default();
        (0..self.nbr_domains.len()).into_iter().for_each(|ii| {
            remote_domain_map.insert(self.nbr_domains[ii], ii);
        });

        for cell_info in self.cell_info_map.values_mut() {
            let domain_gid: usize = cell_info.domain_gid;
            if domain_gid == self.domain_gid { // local cell
                cell_info.cell_index = n_local_cells;
                n_local_cells += 1;
                cell_info.domain_index = self.domain_index;
                cell_info.foreman = self.foreman;
            } 
        }

        for (cell_gid, cell_info) in &self.cell_info_map {
            let domain_gid: usize = cell_info.domain_gid;
            let remote_n_idx = remote_domain_map.get(&domain_gid);
            let tuple_idx: Tuple = grid.cell_idx_to_tuple(*cell_gid);
            let face_nbr = grid.get_face_nbr_gids(*cell_gid);

            for j_cell_gid in face_nbr {
                if let Some(c_info) = self.cell_info_map.get(&j_cell_gid) {
                    if c_info.domain_gid != self.domain_gid {
                        continue
                    }
                    // Comm object sendset insertion
                }
            }
        }

        // Comm object exchange
    }

    fn add_nbrs_to_flood() {}
}
