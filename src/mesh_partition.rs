use std::collections::{HashMap, VecDeque};

use crate::{
    constants::CustomFloat,
    global_fcc_grid::{GlobalFccGrid, Tuple3},
    grid_assignment_object::GridAssignmentObject,
    mc::mc_vector::MCVector,
};

pub type MapType = HashMap<usize, CellInfo>;

/// Structure used to hold cell information.
#[derive(Debug, Clone, Copy, Default)]
pub struct CellInfo {
    /// Domain global identifier
    pub domain_gid: Option<usize>,
    /// Foreman identifier
    pub foreman: Option<usize>,
    /// Domain index
    pub domain_index: Option<usize>,
    /// Cell index
    pub cell_index: Option<usize>,
}

/// Structure used to represent the mesh partition of the space.
/// Holds the different cells' information.
#[derive(Debug, Clone)]
pub struct MeshPartition {
    /// Domain global identifier
    pub domain_gid: usize,
    /// Domain index
    pub domain_index: usize,
    /// Foreman identifier
    pub foreman: usize,
    /// Map linking cell global identifier to thair [CellInfo] structure
    pub cell_info_map: MapType,
    /// List of domain identifiers.
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
    pub fn build_mesh_partition<T: CustomFloat>(
        &mut self,
        grid: &GlobalFccGrid<T>,
        centers: &[MCVector<T>],
        //comm: &mut CommObject,
    ) -> Vec<(usize, usize)> {
        self.assign_cells_to_domain(centers, grid);

        self.build_cell_idx_map(grid)
    }

    /// Internal function that associates cells to domains
    fn assign_cells_to_domain<T: CustomFloat>(
        &mut self,
        domain_center: &[MCVector<T>],
        grid: &GlobalFccGrid<T>,
        //comm: &CommObject,
    ) {
        let mut assigner = GridAssignmentObject::new(domain_center);
        let mut flood_queue: VecDeque<usize> = VecDeque::new();
        let mut wet_cells: Vec<usize> = Vec::new();

        let root = grid.which_cell(&domain_center[self.domain_gid]);

        flood_queue.push_back(root);
        wet_cells.push(root);
        Self::add_nbrs_to_flood(root, grid, &mut flood_queue, &mut wet_cells);

        while !flood_queue.is_empty() {
            let cell_idx = flood_queue.pop_front().unwrap();
            let rr = grid.cell_center(cell_idx);
            let domain = assigner.nearest_center(rr);

            // insert only if the key is absent; in c++ there's no overwriting of keys
            self.cell_info_map.entry(cell_idx).or_insert(CellInfo {
                domain_gid: Some(domain),
                //foreman: Some(self.foreman),
                //domain_index: Some(comm.gid_to_idx[domain]),
                ..Default::default()
            });

            if domain == self.domain_gid {
                // if current cell is in domain, check neighbor
                Self::add_nbrs_to_flood(cell_idx, grid, &mut flood_queue, &mut wet_cells);
            } else if !self.nbr_domains.contains(&domain) {
                // else, keep track of neighbor domains
                self.nbr_domains.push(domain);
            }
        }
    }

    fn build_cell_idx_map<T: CustomFloat>(
        &mut self,
        grid: &GlobalFccGrid<T>,
        //comm: &mut CommObject,
    ) -> Vec<(usize, usize)> {
        let mut remote_cells: Vec<(usize, usize)> = Vec::new();

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
                        if !remote_cells.contains(&(*remote_n_idx, j_cell_gid)) {
                            remote_cells.push((*remote_n_idx, j_cell_gid));
                        }
                    }
                }
            }
        }

        // processing of the return value replaces comm.exchange
        remote_cells
    }

    fn add_nbrs_to_flood<T: CustomFloat>(
        cell_idx: usize,
        grid: &GlobalFccGrid<T>,
        flood_queue: &mut VecDeque<usize>,
        wet_cells: &mut Vec<usize>,
    ) {
        let tt: Tuple3 = grid.cell_idx_to_tuple(cell_idx);

        (-1..2).into_iter().for_each(|ii: i32| {
            (-1..2).into_iter().for_each(|jj: i32| {
                (-1..2).into_iter().for_each(|kk: i32| {
                    if (ii == 0) & (jj == 0) & (kk == 0) {
                        return;
                    }
                    let nbr_tuple = (tt.0 as i32 + ii, tt.1 as i32 + jj, tt.2 as i32 + kk);
                    let snaped_nbr_tuple = grid.snap_turtle(nbr_tuple);
                    let nbr_idx = grid.cell_tuple_to_idx(&snaped_nbr_tuple);
                    if !wet_cells.contains(&nbr_idx) {
                        flood_queue.push_back(nbr_idx);
                        wet_cells.push(nbr_idx);
                    }
                });
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{global_fcc_grid::GlobalFccGrid, mc::mc_vector::MCVector};

    use super::MeshPartition;

    #[test]
    fn partition_building() {
        // simple grid 2*2*2 grid, each cell dim is 1
        let grid = GlobalFccGrid::new(2, 2, 2, 2.0, 2.0, 2.0);
        // 2 symetrical centers
        let c1 = MCVector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let c2 = MCVector {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let centers = vec![c1, c2];
        let domain_gids: Vec<usize> = vec![0, 1];
        let mut partition: Vec<MeshPartition> = Vec::with_capacity(centers.len());
        domain_gids.iter().for_each(|ii| {
            partition.push(MeshPartition::new(*ii, *ii, 0));
        });
        partition.iter_mut().for_each(|part| {
            let remote_cells = part.build_mesh_partition(&grid, &centers);
            // only 2 domains, we can manually process those
            println!("{} remote cells", remote_cells.len());
            remote_cells
                .iter()
                .for_each(|(remote_domain_gid, cell_gid)| {
                    println!("remote domain: {remote_domain_gid}");
                    println!("cell: {:#?}", part.cell_info_map[cell_gid]);
                });
            println!("{part:#?}");
        });
        // Are uninitialized cells still present in the map in QS ?
        partition.iter().for_each(|part| {
            part.cell_info_map.values().for_each(|cell_info| {
                assert!((cell_info.domain_gid.is_some()));
                assert!((cell_info.cell_index.is_some()));
                assert!((cell_info.domain_index.is_some()));
                assert!((cell_info.foreman.is_some()));
            });
        });
    }
}
