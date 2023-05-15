//! Code used for coordinated initialization of domains & meshes

use crate::geometry::mesh_partition::MeshPartition;

/// Structure used to hold a global mapping of [MeshPartition] objects
#[derive(Debug)]
pub struct CommObject {
    pub partition: Vec<MeshPartition>,
    pub gid_to_idx: Vec<usize>,
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
        }
    }
}
