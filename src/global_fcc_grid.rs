use num::{Float, FromPrimitive};

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

    /// Corner offset as tuples? hardcode as field or in a function?
    pub corner_offset: [Tuple4; 14], // change to a CONST
    pub face_offset: [(i32, i32, i32); 6], // change to a CONST
}

impl<T: Float + FromPrimitive> GlobalFccGrid<T> {
    /// Constructor.
    pub fn new(nx: usize, ny: usize, nz: usize, lx: T, ly: T, lz: T) -> Self {
        let tmpx: T = FromPrimitive::from_usize(nx).unwrap();
        let tmpy: T = FromPrimitive::from_usize(ny).unwrap();
        let tmpz: T = FromPrimitive::from_usize(nz).unwrap();

        let corner_offset: [Tuple4; 14] = [
            (0, 0, 0, 0),
            (1, 0, 0, 0),
            (0, 1, 0, 0),
            (1, 1, 0, 0),
            (0, 0, 1, 0),
            (1, 0, 1, 0),
            (0, 1, 1, 0),
            (1, 1, 1, 0),
            (1, 0, 0, 1),
            (0, 0, 0, 1),
            (0, 1, 0, 2),
            (0, 0, 0, 2),
            (0, 0, 1, 3),
            (0, 0, 0, 3),
        ];

        let face_offset: [(i32, i32, i32); 6] = [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];

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
            corner_offset,
            face_offset,
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
        let tt: Tuple = self.cell_idx_to_tuple(idx_cell);
        let r: MCVector<T> = self.node_coord_from_tuple(&(tt.0, tt.1, tt.2, 0));
        r + MCVector {
            x: self.dx / two,
            y: self.dy / two,
            z: self.dz / two,
        }
    }

    /// Converts a cell index to a coordinate tuple.
    pub fn cell_idx_to_tuple(&self, idx_cell: usize) -> Tuple {
        let x = idx_cell % self.nx;
        let tmp = idx_cell / self.nx;
        let y = tmp % self.ny;
        let z = tmp / self.ny;
        (x, y, z)
    }

    /// Converts a cell coordinate tuple to an index.
    pub fn cell_tuple_to_idx(&self, tuple_cell: &Tuple) -> usize {
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
        let z = qy % self.nz;
        let b = qy / self.nz;
        (x, y, z, b)
    }

    /// Returns the global identifiers of ?
    pub fn get_node_gids(&self, cell_gid: usize) -> [usize; 14] {
        let mut node_gid: [usize; 14] = [0; 14];

        let tt: Tuple = self.cell_idx_to_tuple(cell_gid);

        // change to a CONST
        (0..14).into_iter().for_each(|ii| {
            let tmp: Tuple4 = (
                tt.0 + self.corner_offset[ii].0,
                tt.1 + self.corner_offset[ii].1,
                tt.2 + self.corner_offset[ii].2,
                self.corner_offset[ii].3,
            );
            node_gid[ii] = self.node_tuple_to_idx(&tmp);
        });

        node_gid
    }

    /// Returns the global identifiers of ?
    pub fn get_face_nbr_gids(&self, cell_gid: usize) -> [usize; 6] {
        let mut nbr_cell_gid: [usize; 6] = [0; 6];

        let cell_tt = self.cell_idx_to_tuple(cell_gid);
        // change to a CONST
        (0..6).into_iter().for_each(|ii| {
            let face_nbr = (
                cell_tt.0 as i32 + self.face_offset[ii].0,
                cell_tt.1 as i32 + self.face_offset[ii].1,
                cell_tt.2 as i32 + self.face_offset[ii].2,
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
        todo!()
    }

    /// Adjust the tuple value according to bounds
    pub fn snap_turtle(&self, tt: (i32, i32, i32)) -> Tuple {
        todo!()
    }
}
