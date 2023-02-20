use std::{collections::HashMap, marker::PhantomData};

use num::Float;

use crate::{global_fcc_grid::GlobalFccGrid, mc::mc_vector::MCVector};

type MapType = HashMap<u64, CellInfo>;

#[derive(Debug)]
pub struct CellInfo {
    pub domain_gid: usize,
    pub foreman: usize, // ?
    pub domain_index: usize,
    pub cell_index: usize,
}

impl Default for CellInfo {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct MeshPartition<T: Float> {
    domain_gid: usize,
    domain_index: usize,
    foreman: usize,
    pub cell_info_map: MapType,
    nbr_domains: Vec<usize>,
    float_type: PhantomData<T>,
}

impl<T: Float> MeshPartition<T> {
    // Getters
    pub fn domain_gid(&self) -> &usize {
        todo!()
    }
    pub fn domain_index(&self) -> &usize {
        todo!()
    }
    pub fn foreman(&self) -> &usize {
        todo!()
    }
    pub fn nbr_domains(&self) -> &Vec<usize> {
        todo!()
    }

    pub fn build_mesh_partition(grid: &GlobalFccGrid, centers: Vec<MCVector<T>>) {
        todo!()
    }
}
