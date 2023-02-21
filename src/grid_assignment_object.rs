use std::{collections::VecDeque};

use num::Float;

use crate::{mc::mc_vector::MCVector, global_fcc_grid::Tuple};

#[derive(Debug)]
pub struct GridCell {
    pub burned: bool,
    pub my_centers: Vec<u32>,
}

#[derive(Debug)]
pub struct GridAssignmentObject<T: Float> {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub dx: T,
    pub dy: T,
    pub dz: T,

    pub corner: MCVector<T>,
    pub centers: Vec<MCVector<T>>,

    grid: Vec<GridCell>,
    flood_queue: VecDeque<u32>,
    wet_list: VecDeque<u32>,
}

impl<T: Float> GridAssignmentObject<T> {
    pub fn new(centers: &[MCVector<T>]) -> Self {
        todo!()
    }

    pub fn nearest_center(&self, rr: MCVector<T>) -> u32 {
        todo!()
    }

    fn which_cell_tuple(&self, r: MCVector<T>) -> Tuple {
        todo!()
    }

    fn which_cell(&self, r: MCVector<T>) -> usize {
        todo!()
    }

    fn tuple_to_index(&self, t: Tuple) -> usize {
        todo!()
    }

    fn index_to_tuple(&self, idx: usize) -> Tuple {
        todo!()
    }

    fn min_dist2(&self, r: MCVector<T>, cell_idx: usize) -> T {
        todo!()
    }

    fn add_tuple_to_queue(&mut self, t: Tuple) {
        todo!()
    }

    fn add_nbrs_to_queue(&mut self, idx: usize) {
        todo!()
    }
}