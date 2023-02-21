use num::Float;

use crate::mc::mc_vector::MCVector;

pub type Tuple = (usize, usize, usize);
pub type Tuple4 = (usize, usize, usize, usize);

#[derive(Debug)]
pub struct GlobalFccGrid<T: Float> {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,

    pub lx: T,
    pub ly: T,
    pub lz: T,

    pub dx: T,
    pub dy: T,
    pub dz: T,

    pub offset: [Tuple4; 14],
}

impl<T: Float> GlobalFccGrid<T> {
    pub fn new(nx: usize, ny: usize, nz: usize, lx: T, ly: T, lz: T) -> Self {
        todo!()
    }

    pub fn which_cell(&self, r: &MCVector<T>) -> usize {
        todo!()
    }

    pub fn cell_center(&self, idx_cell: usize) -> MCVector<T> {
        todo!()
    }

    pub fn cell_idx_to_tuple(&self, idx_cell: usize) -> Tuple {
        todo!()
    }

    pub fn cell_tuple_to_idx(&self, tuple_cell: &Tuple) -> usize {
        todo!()
    }

    pub fn node_idx(&self, tt: &Tuple4) -> usize {
        todo!()
    }

    pub fn get_node_gids(&self, cell_gid: usize) -> Vec<Tuple4> {
        // replace with array since sized should be fixed ?
        todo!()
    }

    pub fn get_face_nbr_gids(&self, cell_gid: usize) -> Vec<Tuple4> {
        // replace with array since sized should be fixed ?
        todo!()
    }

    pub fn node_coord_from_idx(&self, idx: usize) -> MCVector<T> {
        todo!()
    }

    pub fn node_coord_from_tuple(&self, tt: &Tuple4) -> MCVector<T> {
        self.node_coord_from_idx(self.node_idx(tt))
    }

    pub fn snap_turtle(&self, tt: &Tuple) {
        todo!()
    }
}
