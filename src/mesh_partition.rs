use std::collections::HashMap;

use num::Float;

use crate::{comm_object::CommObject, global_fcc_grid::GlobalFccGrid, mc::mc_vector::MCVector};

pub type MapType = HashMap<usize, CellInfo>;

/// Structure used to hold cell information.
#[derive(Debug, Clone, Copy)]
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
        Self {
            domain_gid,
            domain_index,
            foreman,
            cell_info_map: Default::default(),
            nbr_domains: Default::default(),
        }
    }
    /// Builds the mesh partition.
    pub fn build_mesh_partition<T: Float>(
        &mut self,
        grid: &GlobalFccGrid<T>,
        centers: &[MCVector<T>],
        comm: &mut CommObject,
    ) {
        self.assign_cells_to_domain(centers, grid);

        self.build_cell_idx_map(grid, comm);
    }

    fn assign_cells_to_domain<T: Float>(
        &mut self,
        domain_center: &[MCVector<T>],
        grid: &GlobalFccGrid<T>,
    ) {
    }

    fn build_cell_idx_map<T: Float>(&mut self, grid: &GlobalFccGrid<T>, comm: &mut CommObject) {
        let mut n_local_cells: usize = 0;
        // init a map
        let mut remote_domain_map: HashMap<usize, usize> = Default::default();
        (0..self.nbr_domains.len()).into_iter().for_each(|ii| {
            remote_domain_map.insert(self.nbr_domains[ii], ii);
        });

        let read_map = self.cell_info_map.clone();

        for (cell_gid, cell_info) in &mut self.cell_info_map {
            let domain_gid: usize = cell_info.domain_gid;
            if domain_gid == self.domain_gid {
                // local cell
                cell_info.cell_index = n_local_cells;
                n_local_cells += 1;
                cell_info.domain_index = self.domain_index;
                cell_info.foreman = self.foreman;
            } else {
                let remote_n_idx = remote_domain_map.get(&domain_gid).unwrap();
                let face_nbr = grid.get_face_nbr_gids(*cell_gid);

                for j_cell_gid in face_nbr {
                    if let Some(c_info) = read_map.get(&j_cell_gid) {
                        if c_info.domain_gid != self.domain_gid {
                            continue;
                        }
                        // replace the update to sendSet
                        comm.add_to_send((*remote_n_idx, j_cell_gid));
                    }
                }
            }
        }

        // replace comm.exchange
        comm.send(&mut self.cell_info_map, &self.nbr_domains)
    }

    fn add_nbrs_to_flood() {}
}
