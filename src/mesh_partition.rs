use std::collections::{HashMap, VecDeque};

use num::{Float, FromPrimitive};

use crate::{comm_object::CommObject, global_fcc_grid::{GlobalFccGrid, Tuple}, mc::mc_vector::MCVector, grid_assignment_object::GridAssignmentObject};

pub type MapType = HashMap<usize, CellInfo>;

/// Structure used to hold cell information.
#[derive(Debug, Clone, Copy, Default)]
pub struct CellInfo {
    /// Domain global identifier.
    pub domain_gid: Option<usize>,
    /// ?
    pub foreman: Option<usize>, // ?
    /// Domain index?
    pub domain_index: Option<usize>,
    /// Cell index.
    pub cell_index: Option<usize>,
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
    pub fn build_mesh_partition<T: Float + FromPrimitive>(
        &mut self,
        grid: &GlobalFccGrid<T>,
        centers: &[MCVector<T>],
        comm: &mut CommObject,
    ) {
        self.assign_cells_to_domain(centers, grid);

        self.build_cell_idx_map(grid, comm);
    }

    fn assign_cells_to_domain<T: Float + FromPrimitive>(
        &mut self,
        domain_center: &[MCVector<T>],
        grid: &GlobalFccGrid<T>,
    ) {
        let mut assigner = GridAssignmentObject::new(domain_center);
        let mut flood_queue: VecDeque<usize> = VecDeque::new();
        let mut wet_cells: Vec<usize> = Vec::new();
        let mut remote_domain: Vec<usize> = Vec::new();

        let root = grid.which_cell(&domain_center[self.domain_gid]);

        flood_queue.push_back(root);
        wet_cells.push(root);
        Self::add_nbrs_to_flood(root, grid, &mut flood_queue, &mut wet_cells);

        while !flood_queue.is_empty() {
            let cell_idx = flood_queue.pop_front().unwrap();
            let rr = grid.cell_center(cell_idx);
            let domain = assigner.nearest_center(rr);

            self.cell_info_map.insert(cell_idx, CellInfo { domain_gid: Some(domain), ..Default::default() });

            if domain == self.domain_gid {
                Self::add_nbrs_to_flood(cell_idx, grid, &mut flood_queue, &mut wet_cells);
            } else {
                remote_domain.push(domain);
            }

            self.nbr_domains.extend(remote_domain.iter());
        }

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
            let domain_gid: usize = cell_info.domain_gid.unwrap();
            if domain_gid == self.domain_gid {
                // local cell
                cell_info.cell_index = Some(n_local_cells);
                n_local_cells += 1;
                cell_info.domain_index = Some(self.domain_index);
                cell_info.foreman = Some(self.foreman);
            } else {
                let remote_n_idx = remote_domain_map.get(&domain_gid).unwrap();
                let face_nbr = grid.get_face_nbr_gids(*cell_gid);

                for j_cell_gid in face_nbr {
                    if let Some(c_info) = read_map.get(&j_cell_gid) {
                        if c_info.domain_gid != Some(self.domain_gid) {
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

    fn add_nbrs_to_flood<T: Float>(cell_idx: usize, grid: &GlobalFccGrid<T>, flood_queue: &mut VecDeque<usize>, wet_cells: &mut Vec<usize>) {
        let tt: Tuple = grid.cell_idx_to_tuple(cell_idx);
        (0..3).into_iter().for_each(|ii| {
            (0..3).into_iter().for_each(|jj| {
                (0..3).into_iter().for_each(|kk| {
                    if (ii==0) & (jj==0) & (kk==0) {
                        return;
                    }
                    let nbr_tuple = (tt.0 + ii, tt.1 + jj, tt.2 + kk);
                    grid.snap_turtle(&nbr_tuple);
                    let nbr_idx = grid.cell_tuple_to_idx(&nbr_tuple);
                    if !wet_cells.contains(&nbr_idx) {
                        flood_queue.push_back(nbr_idx);
                        wet_cells.push(nbr_idx);
                    }
                });
            });
        });
    }
}
