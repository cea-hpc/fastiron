use crate::mesh_partition::{MapType, MeshPartition};

/// Structure used to hold a global mapping of [MeshPartition] objects
#[derive(Debug)]
pub struct CommObject {
    pub partition: Vec<MeshPartition>,
    pub gid_to_idx: Vec<usize>,
    pub s_list: Vec<(usize, usize)>,
}

impl CommObject {
    /// Constructor.
    pub fn new(partition: &[MeshPartition]) -> Self {
        let mut gid_to_idx: Vec<usize> = Vec::with_capacity(partition.len());

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

    /// Add the specified data to the internal send list for future processing.
    pub fn add_to_send(&mut self, (remote_domain_idx, cell_gid): (usize, usize)) {
        self.s_list.push((remote_domain_idx, cell_gid));
    }

    /// Process the send list.
    pub fn send(&mut self, cell_info_map: &mut MapType, nbr_domain: &[usize]) {
        for (remote_domain_idx, cell_gid) in &self.s_list {
            let target_domain_gid = nbr_domain[*remote_domain_idx];
            let target_partition = &mut self.partition[self.gid_to_idx[target_domain_gid]];
            let cell_to_send = cell_info_map.get(cell_gid).unwrap();
            target_partition
                .cell_info_map
                .insert(*cell_gid, *cell_to_send);
        }
        self.s_list.clear()
    }
}
