use num::{zero, FromPrimitive};

use crate::{
    constants::{
        mesh::{CORNER_OFFSET, FACE_OFFSET, N_FACES, N_POINTS_INTERSEC},
        CustomFloat, Tuple3, Tuple4,
    },
    data::mc_vector::MCVector,
};

/// Structure representing the spatial grid of the problem.
#[derive(Debug)]
pub struct GlobalFccGrid<T: CustomFloat> {
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
}

impl<T: CustomFloat> GlobalFccGrid<T> {
    /// Constructor.
    pub fn new(nx: usize, ny: usize, nz: usize, lx: T, ly: T, lz: T) -> Self {
        let tmpx: T = FromPrimitive::from_usize(nx).unwrap();
        let tmpy: T = FromPrimitive::from_usize(ny).unwrap();
        let tmpz: T = FromPrimitive::from_usize(nz).unwrap();

        Self {
            nx,
            ny,
            nz,
            lx,
            ly,
            lz,
            dx: lx / tmpx,
            dy: ly / tmpy,
            dz: lz / tmpz,
        }
    }

    /// Returns the index of the cell the coordinates belong to.
    pub fn which_cell(&self, r: &MCVector<T>) -> usize {
        let ix = r.x / self.dx;
        let iy = r.y / self.dy;
        let iz = r.z / self.dz;
        self.cell_tuple_to_idx(&(
            ix.to_usize().unwrap(),
            iy.to_usize().unwrap(),
            iz.to_usize().unwrap(),
        ))
    }

    /// Returns the center of the given cell.
    pub fn cell_center(&self, idx_cell: usize) -> MCVector<T> {
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let tt: Tuple3 = self.cell_idx_to_tuple(idx_cell);
        let r: MCVector<T> = self.node_coord_from_tuple(&(tt.0, tt.1, tt.2, 0));
        r + MCVector {
            x: self.dx / two,
            y: self.dy / two,
            z: self.dz / two,
        }
    }

    /// Converts a cell index to a coordinate tuple.
    pub fn cell_idx_to_tuple(&self, idx_cell: usize) -> Tuple3 {
        let x = idx_cell % self.nx;
        let tmp = idx_cell / self.nx;
        let y = tmp % self.ny;
        let z = tmp / self.ny;
        (x, y, z)
    }

    /// Converts a cell coordinate tuple to an index.
    pub fn cell_tuple_to_idx(&self, tuple_cell: &Tuple3) -> usize {
        tuple_cell.0 + self.nx * (tuple_cell.1 + self.ny * tuple_cell.2)
    }

    /// Converts a node coordinate tuple to an index.
    pub fn node_tuple_to_idx(&self, tt: &Tuple4) -> usize {
        // nx, ny, nz are init with a +1 value in original code;
        // nb is init at 4 but unused
        tt.0 + (self.nx + 1) * (tt.1 + (self.ny + 1) * (tt.2 + (self.nz + 1) * tt.3))
    }

    /// Converts a node index to a coordinate tuple.
    pub fn node_idx_to_tuple(&self, idx: usize) -> Tuple4 {
        // nx, ny, nz are init with a +1 value in original code;
        // nb is init at 4 but unused
        let x = idx % (self.nx + 1);
        let qx = idx / (self.nx + 1);
        let y = qx % (self.ny + 1);
        let qy = qx / (self.ny + 1);
        let z = qy % (self.nz + 1);
        let b = qy / (self.nz + 1);
        (x, y, z, b)
    }

    /// Returns the global identifiers of the nodes of the specified cell.
    pub fn get_node_gids(&self, cell_gid: usize) -> [usize; N_POINTS_INTERSEC] {
        let mut node_gid: [usize; N_POINTS_INTERSEC] = [0; N_POINTS_INTERSEC];

        let tt: Tuple3 = self.cell_idx_to_tuple(cell_gid);

        (0..N_POINTS_INTERSEC).for_each(|ii| {
            let tmp: Tuple4 = (
                tt.0 + CORNER_OFFSET[ii].0,
                tt.1 + CORNER_OFFSET[ii].1,
                tt.2 + CORNER_OFFSET[ii].2,
                CORNER_OFFSET[ii].3,
            );
            node_gid[ii] = self.node_tuple_to_idx(&tmp);
        });

        node_gid
    }

    /// Returns the global identifiers of the faces of the specified cell.
    pub fn get_face_nbr_gids(&self, cell_gid: usize) -> [usize; N_FACES] {
        let mut nbr_cell_gid: [usize; N_FACES] = [0; N_FACES];

        let cell_tt = self.cell_idx_to_tuple(cell_gid);

        (0..N_FACES).for_each(|ii| {
            let face_nbr = (
                cell_tt.0 as i32 + FACE_OFFSET[ii].0,
                cell_tt.1 as i32 + FACE_OFFSET[ii].1,
                cell_tt.2 as i32 + FACE_OFFSET[ii].2,
            );
            let snaped_face_nbr = self.snap_turtle(face_nbr);
            nbr_cell_gid[ii] = self.cell_tuple_to_idx(&snaped_face_nbr);
        });

        nbr_cell_gid
    }

    /// Returns a node's coordinate from its index.
    pub fn node_coord_from_idx(&self, idx: usize) -> MCVector<T> {
        self.node_coord_from_tuple(&self.node_idx_to_tuple(idx))
    }

    /// Returns a node's coordinate from its tuple.
    pub fn node_coord_from_tuple(&self, tt: &Tuple4) -> MCVector<T> {
        let two: T = FromPrimitive::from_f64(2.0).unwrap();
        let basis_offset: [MCVector<T>; 4] = [
            MCVector::default(),
            MCVector {
                x: zero(),
                y: self.dy / two,
                z: self.dz / two,
            },
            MCVector {
                x: self.dx / two,
                y: zero(),
                z: self.dz / two,
            },
            MCVector {
                x: self.dx / two,
                y: self.dy / two,
                z: zero(),
            },
        ];
        let tx: T = FromPrimitive::from_usize(tt.0).unwrap();
        let ty: T = FromPrimitive::from_usize(tt.1).unwrap();
        let tz: T = FromPrimitive::from_usize(tt.2).unwrap();

        MCVector {
            x: tx * self.dx,
            y: ty * self.dy,
            z: tz * self.dz,
        } + basis_offset[tt.3]
    }

    /// Adjust the tuple value according to bounds.
    pub fn snap_turtle(&self, tt: (i32, i32, i32)) -> Tuple3 {
        // set tt such that (0 <= tt.* < n*)
        (
            (tt.0.max(0) as usize).min(self.nx - 1),
            (tt.1.max(0) as usize).min(self.ny - 1),
            (tt.2.max(0) as usize).min(self.nz - 1),
        )
    }
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    use super::GlobalFccGrid;

    #[test]
    fn snap_turtle() {
        let grid = GlobalFccGrid::new(3, 3, 3, 9.0, 9.0, 9.0);
        let t0: (i32, i32, i32) = (0, 2, 1); // in bounds
        let t1: (i32, i32, i32) = (3, -1, -2); // out of bounds
        assert_eq!(grid.snap_turtle(t0), (0, 2, 1));
        assert_eq!(grid.snap_turtle(t1), (2, 0, 0));
    }
}
