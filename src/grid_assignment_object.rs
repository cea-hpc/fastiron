use std::collections::VecDeque;

use num::Float;

use crate::{global_fcc_grid::Tuple, mc::mc_vector::MCVector};

/// Internal structure of [GridAssignmentObject].
/// Represents a cell.
#[derive(Debug)]
pub struct GridCell {
    pub burned: bool,
    pub my_centers: Vec<u32>,
}

/// Structure used to "locate" vectors in the grid.
#[derive(Debug)]
pub struct GridAssignmentObject<T: Float> {
    /// Number of cells along the x axis
    pub nx: usize,
    /// Number of cells along the y axis
    pub ny: usize,
    /// Number of cells along the z axis
    pub nz: usize,
    /// Size of a mesh cell along the x axis (cm)
    pub dx: T,
    /// Size of a mesh cell along the y axis (cm)
    pub dy: T,
    /// Size of a mesh cell along the z axis (cm)
    pub dz: T,

    /// List of corners.
    pub corner: MCVector<T>,
    /// List of centers.
    pub centers: Vec<MCVector<T>>,

    /// List of cells.
    grid: Vec<GridCell>,
    /// ?
    flood_queue: VecDeque<u32>,
    /// ?
    wet_list: VecDeque<u32>,
}

impl<T: Float> GridAssignmentObject<T> {
    /// Constructor.
    pub fn new(centers: &[MCVector<T>]) -> Self {
        todo!()
    }

    /// Returns the closest center to a given coordinate.
    pub fn nearest_center(&self, rr: MCVector<T>) -> u32 {
        todo!()
    }

    /// Returns the tuple of the cell the coordinate belongs to.
    fn which_cell_tuple(&self, r: MCVector<T>) -> Tuple {
        todo!()
    }

    /// Returns the index of the cell the coordinate belongs to.
    fn which_cell(&self, r: MCVector<T>) -> usize {
        todo!()
    }

    /// Converts a cell tuple to its index.
    fn tuple_to_index(&self, t: Tuple) -> usize {
        todo!()
    }

    /// Converts a cell index to its tuple.
    fn index_to_tuple(&self, idx: usize) -> Tuple {
        todo!()
    }

    /// Finds a lower bound of the squared distance from the point
    /// r to the cell with index cell_idx.
    fn min_dist2(&self, r: MCVector<T>, cell_idx: usize) -> T {
        todo!()
    }

    /// ?
    fn add_tuple_to_queue(&mut self, t: Tuple) {
        todo!()
    }

    /// ?
    fn add_nbrs_to_queue(&mut self, idx: usize) {
        todo!()
    }
}
