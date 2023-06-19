//! Code used to build and model the mesh of the problem
//!
//!

use std::collections::VecDeque;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    constants::{CustomFloat, Tuple3},
    data::mc_vector::MCVector,
};

use super::{global_fcc_grid::GlobalFccGrid, grid_assignment_object::GridAssignmentObject};

type MapType = FxHashMap<usize, CellInfo>;

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
///
/// Holds all the different cells' information.
#[derive(Debug, Clone)]
pub struct MeshPartition {
    /// Domain global identifier.
    pub domain_gid: usize,
    /// Foreman identifier.
    pub foreman: usize,
    /// Map linking cell global identifier to their [CellInfo] structure
    pub cell_info_map: MapType,
    /// List of neighboring domain identifiers. **Should be replaced by a set**.
    pub nbr_domains: FxHashSet<usize>,
}

impl MeshPartition {
    /// Constructor. The structure is **not** ready to be used directly.
    pub fn new(domain_gid: usize, foreman: usize) -> Self {
        Self {
            domain_gid,
            foreman,
            cell_info_map: Default::default(),
            nbr_domains: Default::default(),
        }
    }
    /// Builds the mesh partition. This method needs to be called after the
    /// constructor.
    pub fn build_mesh_partition<T: CustomFloat>(
        &mut self,
        grid: &GlobalFccGrid<T>,
        centers: &[MCVector<T>],
    ) -> FxHashSet<(usize, usize)> {
        self.assign_cells_to_domain(centers, grid);

        self.build_cell_idx_map(grid)
    }

    /// Internal function that associates cells to domains
    fn assign_cells_to_domain<T: CustomFloat>(
        &mut self,
        domain_center: &[MCVector<T>],
        grid: &GlobalFccGrid<T>,
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
                ..Default::default()
            });

            if domain == self.domain_gid {
                // if current cell is in domain, check neighbor
                Self::add_nbrs_to_flood(cell_idx, grid, &mut flood_queue, &mut wet_cells);
            } else {
                // else, keep track of neighbor domains
                self.nbr_domains.insert(domain);
            }
        }
    }

    fn build_cell_idx_map<T: CustomFloat>(
        &mut self,
        grid: &GlobalFccGrid<T>,
    ) -> FxHashSet<(usize, usize)> {
        let mut remote_cells: FxHashSet<(usize, usize)> = Default::default();

        let mut n_local_cells: usize = 0;

        let read_map = self.cell_info_map.clone();

        for (cell_gid, cell_info) in &mut self.cell_info_map {
            let domain_gid: usize = cell_info.domain_gid.unwrap();
            if domain_gid == self.domain_gid {
                // local cell
                cell_info.cell_index = Some(n_local_cells);
                n_local_cells += 1;
                cell_info.domain_index = Some(self.domain_gid);
                cell_info.foreman = Some(self.foreman);
            } else {
                let face_nbr = grid.get_face_nbr_gids(*cell_gid);

                for j_cell_gid in face_nbr {
                    if let Some(c_info) = read_map.get(&j_cell_gid) {
                        if c_info.domain_gid != Some(self.domain_gid) {
                            continue;
                        }
                        // replace the update to sendSet
                        remote_cells.insert((domain_gid, j_cell_gid));
                    }
                }
            }
        }

        // processing of the return value replaces comm.exchange
        // remote cells are the cells of the CURRENT partition that
        // are neighbors to the neighbor partition
        // hence they are supposed to be inserted in the corresponding
        // neighbor partition as they are (they contain correct info).
        remote_cells
    }

    fn add_nbrs_to_flood<T: CustomFloat>(
        cell_idx: usize,
        grid: &GlobalFccGrid<T>,
        flood_queue: &mut VecDeque<usize>,
        wet_cells: &mut Vec<usize>,
    ) {
        let tt: Tuple3 = grid.cell_idx_to_tuple(cell_idx);
        const NBR_COORDS: [(i32, i32, i32); 26] = [
            // (-1, x, y)
            (-1, -1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, 0, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 1, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            // (0, x, y) except (0, 0, 0)
            (0, -1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, 0, -1),
            (0, 0, 1),
            (0, 1, -1),
            (0, 1, 0),
            (0, 1, 1),
            // (1, x, y)
            (1, -1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, 0, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, -1),
            (1, 1, 0),
            (1, 1, 1),
        ];

        NBR_COORDS
            .iter()
            .map(|offset| {
                let snaped = grid.snap_turtle((
                    tt.0 as i32 + offset.0,
                    tt.1 as i32 + offset.1,
                    tt.2 as i32 + offset.2,
                ));
                grid.cell_tuple_to_idx(&snaped)
            })
            .for_each(|nbr_idx| {
                if !wet_cells.contains(&nbr_idx) {
                    flood_queue.push_back(nbr_idx);
                    wet_cells.push(nbr_idx);
                }
            });
    }
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {

    use crate::{data::mc_vector::MCVector, geometry::global_fcc_grid::GlobalFccGrid};

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
            partition.push(MeshPartition::new(*ii, 0));
        });

        (0..partition.len()).for_each(|part_idx| {
            let remote_cells = partition[part_idx].build_mesh_partition(&grid, &centers);
            // only 2 domains, we can manually process those; gids and indexes are coherent
            println!("{} remote cells", remote_cells.len());
            // remote cells are a special case where we want to overwrite the target map's entry
            remote_cells
                .iter()
                .for_each(|(remote_domain_gid, cell_gid)| {
                    let cell_to_insert = partition[part_idx].cell_info_map[cell_gid];
                    partition[*remote_domain_gid]
                        .cell_info_map
                        .insert(*cell_gid, cell_to_insert);

                    println!("remote domain: {remote_domain_gid}");
                    println!("cell (gid {cell_gid}): {cell_to_insert:#?}");
                });

            println!("{:#?}", partition[part_idx]);
            println!();
        });

        // NOTE: only the belonging and neighboring cells are initialized
        // NOTE: is there a way to test this or remove non neighboring cells? is it worth it?
        // for this simple case, non neighboring cell are gid 0 in domain 1 and
        // gid 7 in domain 0
        partition.iter().for_each(|part| {
            part.cell_info_map.iter().for_each(|(cell_gid, cell_info)| {
                if ((*cell_gid == 0) & (part.domain_gid == 1))
                    || ((*cell_gid == 7) & (part.domain_gid == 0))
                {
                    assert!(cell_info.cell_index.is_none());
                    assert!((cell_info.domain_index.is_none()));
                    assert!((cell_info.foreman.is_none()));
                    return;
                }
                assert!((cell_info.domain_gid.is_some()));
                assert!((cell_info.cell_index.is_some()));
                assert!((cell_info.domain_index.is_some()));
                assert!((cell_info.foreman.is_some()));
            });
        });
    }
}
