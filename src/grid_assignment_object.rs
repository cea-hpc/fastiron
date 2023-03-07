use std::collections::VecDeque;

use num::{zero, Float, FromPrimitive};

use crate::{global_fcc_grid::Tuple3, mc::mc_vector::MCVector, physical_constants::TINY_FLOAT};

/// Internal structure of [GridAssignmentObject].
/// Represents a cell.
#[derive(Debug, Default)] // default value of bool is false
pub struct GridCell {
    pub burned: bool,
    pub my_centers: Vec<usize>,
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
    /// Internal queue used when browsing through the cells
    flood_queue: VecDeque<usize>,
    /// Internal queue used when browsing through the cells
    wet_list: VecDeque<usize>,
}

impl<T: Float + FromPrimitive> GridAssignmentObject<T> {
    /// Constructor.
    pub fn new(centers: &[MCVector<T>]) -> Self {
        // sets the length scale of the grid cells
        let centers_per_cell: T = FromPrimitive::from_usize(5).unwrap();
        let n_centers: T = FromPrimitive::from_usize(centers.len()).unwrap();
        let one: T = FromPrimitive::from_f64(1.0).unwrap();

        let mut min_coords = centers[0];
        let mut max_coords = centers[0];
        centers.iter().for_each(|vv| {
            min_coords.x = min_coords.x.min(vv.x);
            min_coords.y = min_coords.x.min(vv.y);
            min_coords.z = min_coords.x.min(vv.z);
            max_coords.x = max_coords.x.max(vv.x);
            max_coords.y = max_coords.x.max(vv.y);
            max_coords.z = max_coords.x.max(vv.z);
        });

        let lx = one.max(max_coords.x - min_coords.x);
        let ly = one.max(max_coords.y - min_coords.y);
        let lz = one.max(max_coords.z - min_coords.z);
        let dim: T = n_centers / (centers_per_cell * lx * ly * lz).cbrt();
        let nx = one.max((dim * lx).floor());
        let ny = one.max((dim * ly).floor());
        let nz = one.max((dim * lz).floor());
        let dx = lx / nx;
        let dy = ly / ny;
        let dz = lz / nz;

        let n_cells: usize = (nx * ny * nz).to_usize().unwrap();
        let grid: Vec<GridCell> = Vec::with_capacity(n_cells);

        let mut gao: GridAssignmentObject<T> = Self {
            nx: nx.to_usize().unwrap(),
            ny: ny.to_usize().unwrap(),
            nz: nz.to_usize().unwrap(),
            dx,
            dy,
            dz,
            corner: min_coords,
            centers: centers.to_vec(),
            grid,
            flood_queue: Default::default(),
            wet_list: Default::default(),
        };

        (0..centers.len()).into_iter().for_each(|center_idx| {
            let cell_idx = gao.which_cell(centers[center_idx]);
            gao.grid[cell_idx].my_centers.push(center_idx);
        });

        gao
    }

    /// Returns the closest center to a given coordinate.
    pub fn nearest_center(&mut self, rr: MCVector<T>) -> usize {
        let mut r2_min: T = FromPrimitive::from_f64(1e300).unwrap();
        let tiny_f: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
        let mut center_min: Option<usize> = None;

        self.add_tuple_to_queue(self.which_cell_tuple(rr));

        while !self.flood_queue.is_empty() {
            // next cell to check
            let cell_idx: usize = self.flood_queue.pop_front().unwrap();
            // if cell is too far away, dont even try
            if self.min_dist2(rr, cell_idx) > r2_min {
                continue;
            }
            for center_idx in &self.grid[cell_idx].my_centers {
                let center_r = self.centers[*center_idx];
                let r2: T = (rr - center_r).dot(&(rr - center_r));
                if (r2 - r2_min).abs() < tiny_f {
                    // r2 == r2_min
                    center_min.map(|m| m.min(*center_idx));
                }
                if r2 < r2_min {
                    // replace another threshold test?
                    r2_min = r2;
                    center_min = Some(*center_idx);
                }
            }
            self.add_nbrs_to_queue(cell_idx);
        }

        while !self.wet_list.is_empty() {
            self.grid[self.wet_list.pop_front().unwrap()].burned = false;
        }

        assert!(center_min.is_some());
        center_min.unwrap()
    }

    /// Returns the tuple of the cell the coordinate belongs to.
    fn which_cell_tuple(&self, r: MCVector<T>) -> Tuple3 {
        let mut ix: usize = ((r.x - self.corner.x) / self.dx)
            .floor()
            .max(zero())
            .to_usize()
            .unwrap();
        let mut iy: usize = ((r.y - self.corner.y) / self.dy)
            .floor()
            .max(zero())
            .to_usize()
            .unwrap();
        let mut iz: usize = ((r.z - self.corner.z) / self.dz)
            .floor()
            .max(zero())
            .to_usize()
            .unwrap();
        ix = ix.min(self.nx - 1);
        iy = iy.min(self.ny - 1);
        iz = iz.min(self.nz - 1);

        (ix, iy, iz)
    }

    /// Returns the index of the cell the coordinate belongs to.
    fn which_cell(&self, r: MCVector<T>) -> usize {
        self.tuple_to_index(&self.which_cell_tuple(r))
    }

    /// Converts a cell tuple to its index.
    fn tuple_to_index(&self, t: &Tuple3) -> usize {
        t.0 + self.nx * (t.1 + self.ny * t.2)
    }

    /// Converts a cell index to its tuple.
    fn index_to_tuple(&self, idx: usize) -> Tuple3 {
        let ix: usize = idx % self.nx;
        let tmp: usize = idx / self.nx;
        let iy: usize = tmp % self.ny;
        let iz: usize = tmp / self.ny;
        (ix, iy, iz)
    }

    /// Finds a lower bound of the squared distance from the point
    /// r to the cell with index cell_idx.
    fn min_dist2(&self, r: MCVector<T>, cell_idx: usize) -> T {
        let r_idx: Tuple3 = self.which_cell_tuple(r);
        let tuple_idx: Tuple3 = self.index_to_tuple(cell_idx);

        let rx: T = (self.dx
            * (FromPrimitive::from_usize(tuple_idx.0.abs_diff(r_idx.0) - 1)).unwrap())
        .max(zero());
        let ry: T = (self.dy
            * (FromPrimitive::from_usize(tuple_idx.1.abs_diff(r_idx.1) - 1)).unwrap())
        .max(zero());
        let rz: T = (self.dz
            * (FromPrimitive::from_usize(tuple_idx.2.abs_diff(r_idx.2) - 1)).unwrap())
        .max(zero());

        rx * rx + ry * ry + rz * rz
    }

    /// ?
    fn add_tuple_to_queue(&mut self, t: Tuple3) {
        let idx: usize = self.tuple_to_index(&t);
        if self.grid[idx].burned {
            return;
        }
        self.flood_queue.push_back(idx);
        self.wet_list.push_back(idx);
        self.grid[idx].burned = true
    }

    /// Add valid tuple to internal queues.
    fn add_nbrs_to_queue(&mut self, cell_idx: usize) {
        let tuple_idx: Tuple3 = self.index_to_tuple(cell_idx);
        // on x
        if tuple_idx.0 + 1 < self.nx {
            let mut tmp: Tuple3 = tuple_idx;
            tmp.0 += 1;
            self.add_tuple_to_queue(tmp);
        }
        if tuple_idx.0 > 0 {
            let mut tmp: Tuple3 = tuple_idx;
            tmp.0 -= 1;
            self.add_tuple_to_queue(tmp);
        }
        // on y
        if tuple_idx.1 + 1 < self.ny {
            let mut tmp: Tuple3 = tuple_idx;
            tmp.1 += 1;
            self.add_tuple_to_queue(tmp);
        }
        if tuple_idx.1 > 0 {
            let mut tmp: Tuple3 = tuple_idx;
            tmp.1 -= 1;
            self.add_tuple_to_queue(tmp);
        }
        // on z
        if tuple_idx.2 + 1 < self.nz {
            let mut tmp: Tuple3 = tuple_idx;
            tmp.2 += 1;
            self.add_tuple_to_queue(tmp);
        }
        if tuple_idx.2 > 0 {
            let mut tmp: Tuple3 = tuple_idx;
            tmp.2 -= 1;
            self.add_tuple_to_queue(tmp);
        }
    }
}
