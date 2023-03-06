use crate::mesh_partition::{MeshPartition, MapType};

#[derive(Debug)]
pub struct CommObject {
    pub partition: Vec<MeshPartition>,
    pub gid_to_idx: Vec<usize>,
}

impl CommObject {
    pub fn new(partition: Vec<MeshPartition>) -> Self {
        let mut gid_to_idx: Vec<usize> = Vec::with_capacity(partition.len());

        (0..partition.len()).for_each(|ii| {
            assert!(partition[ii].domain_gid < partition.len());
            gid_to_idx[partition[ii].domain_gid] = ii;
        });

        Self { partition, gid_to_idx }
    }

    pub fn send(&mut self, cell_info_map: &mut MapType, nbr_domain: &[usize], (remote_domain_idx, cell_gid): (usize, usize)) {
        let target_domain_gid = nbr_domain[remote_domain_idx];
        let target_partition = &mut self.partition[self.gid_to_idx[target_domain_gid]];
        let cell_to_send = cell_info_map.get(&cell_gid).unwrap();
        target_partition.cell_info_map.insert(cell_gid, *cell_to_send);
    }
}