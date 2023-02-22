use num::Float;

use crate::mc::mc_vector::MCVector;

/// Custom alias for readability. Might change
/// name so that it doesn't overlap with the
/// primitive.
pub type Tuple = (usize, usize, usize);
/// Custom alias for readability. Might change
/// name so that it doesn't overlap with the
/// primitive.
pub type Tuple4 = (usize, usize, usize, usize);

/// Structure representing the spatial grid of the problem.
#[derive(Debug)]
pub struct GlobalFccGrid<T: Float> {
    /// Number of cells along the x axis
    pub nx: usize,
    /// Number of cells along the y axis
    pub ny: usize,
    /// Number of cells along the z axis
    pub nz: usize,

    /// Size of the problem along the x axis (cm)
    pub lx: T,
    /// Size of the problem along the y axis (cm)
    pub ly: T,
    /// Size of the problem along the z axis (cm)
    pub lz: T,

    /// Size of a mesh cell along the x axis (cm)
    pub dx: T,
    /// Size of a mesh cell along the y axis (cm)
    pub dy: T,
    /// Size of a mesh cell along the z axis (cm)
    pub dz: T,

    /// Corner offset as tuples?
    pub offset: [Tuple4; 14],
}

impl<T: Float> GlobalFccGrid<T> {
    /// Constructor.
    pub fn new(nx: usize, ny: usize, nz: usize, lx: T, ly: T, lz: T) -> Self {
        todo!()
    }

    /// Returns the index of the cell the coordinates belong to.
    pub fn which_cell(&self, r: &MCVector<T>) -> usize {
        todo!()
    }

    /// Returns the center of the given cell.
    pub fn cell_center(&self, idx_cell: usize) -> MCVector<T> {
        todo!()
    }

    /// Converts a cell index to a coordinate tuple.
    pub fn cell_idx_to_tuple(&self, idx_cell: usize) -> Tuple {
        todo!()
    }

    /// Converts a cell coordinate tuple to an index.
    pub fn cell_tuple_to_idx(&self, tuple_cell: &Tuple) -> usize {
        todo!()
    }

    /// Converts a node index to a coordinate tuple.
    pub fn node_idx(&self, tt: &Tuple4) -> usize {
        todo!()
    }

    /// Returns the global identifiers of ?
    pub fn get_node_gids(&self, cell_gid: usize) -> Vec<u64> {
        // replace with array since sized should be fixed ?
        todo!()
    }

    /// Returns the global identifiers of ?
    pub fn get_face_nbr_gids(&self, cell_gid: usize) -> Vec<u64> {
        // replace with array since sized should be fixed ?
        todo!()
    }

    /// Returns a node's coordinate from its index.
    pub fn node_coord_from_idx(&self, idx: usize) -> MCVector<T> {
        todo!()
    }

    /// Returns a node's coordinate from its tuple.
    pub fn node_coord_from_tuple(&self, tt: &Tuple4) -> MCVector<T> {
        self.node_coord_from_idx(self.node_idx(tt))
    }

    /// ?
    pub fn snap_turtle(&self, tt: &Tuple) {
        todo!()
    }
}
