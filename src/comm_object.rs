use crate::{mesh_partition::{MapType, MeshPartition, CellInfo}, global_fcc_grid::GlobalFccGrid, constants::CustomFloat, mc::mc_vector::MCVector};

/// Structure used to hold a global mapping of [MeshPartition] objects
#[derive(Debug)]
pub struct CommObject {
    pub partition: Vec<MeshPartition>,
    pub gid_to_idx: Vec<usize>,
    pub s_list: Vec<(usize, usize, CellInfo)>,
}

impl CommObject {
    /// Constructor.
    pub fn new(partition: &[MeshPartition]) -> Self {
        let mut gid_to_idx: Vec<usize> = vec![0; partition.len()];

        (0..partition.len()).for_each(|ii| {
            assert!(partition[ii].domain_gid < partition.len());
            gid_to_idx[partition[ii].domain_gid] = ii;
        });
        let p = partition.to_vec();

        Self {
            partition: p,
            gid_to_idx,
            s_list: Vec::new(),
        }
    }

    pub fn build_mesh_partition<T: CustomFloat>(&mut self, global_grid: &GlobalFccGrid<T>, domain_centers: &[MCVector<T>]) {
        self.partition.iter_mut().for_each(|mesh_p| {
            let remote_cells = mesh_p.build_mesh_partition(global_grid, domain_centers);
            self.s_list.extend(remote_cells.iter());
        });
        self.send();
    }

    /// Add the specified data to the internal send list for future processing.
    pub fn add_to_send(&mut self, (remote_domain_gid, cell_gid, cell_info): (usize, usize, CellInfo)) {
        self.s_list.push((remote_domain_gid, cell_gid, cell_info));
    }

    /// Process the send list.
    pub fn send(&mut self) {
        for (remote_domain_gid, cell_gid, cell_info) in &self.s_list {
            let target_partition = &mut self.partition[self.gid_to_idx[*remote_domain_gid]];
            let cell_to_send = cell_info; //cell_info_map.get(cell_gid).unwrap();
            assert!(cell_to_send.domain_index.is_some());
            assert!(cell_to_send.cell_index.is_some());
            target_partition
                .cell_info_map
                .insert(*cell_gid, *cell_to_send);
        }
        self.s_list.clear()
    }
}
