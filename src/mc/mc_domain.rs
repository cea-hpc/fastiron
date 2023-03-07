use std::collections::HashMap;

use num::{one, zero, Float, FromPrimitive};

use crate::{
    bulk_storage::BulkStorage,
    decomposition_object::DecompositionObject,
    global_fcc_grid::GlobalFccGrid,
    mesh_partition::{CellInfo, MeshPartition},
    parameters::{GeometryParameters, Parameters},
};

use super::{
    mc_cell_state::MCCellState,
    mc_facet_adjacency::{MCFacetAdjacency, MCFacetAdjacencyCell, MCSubfacetAdjacencyEvent},
    mc_facet_geometry::{MCFacetGeometryCell, MCGeneralPlane},
    mc_location::MCLocation,
    mc_vector::MCVector,
};

#[derive(Debug)]
struct FaceInfo {
    pub event: MCSubfacetAdjacencyEvent,
    pub cell_info: CellInfo,
    pub nbr_idx: usize,
}

const NODE_INDIRECT: [[usize; 3]; 24] = [
    [1, 3, 8],
    [3, 7, 8],
    [7, 5, 8],
    [5, 1, 8],
    [0, 4, 9],
    [4, 6, 9],
    [6, 2, 9],
    [2, 0, 9],
    [3, 2, 10],
    [2, 6, 10],
    [6, 7, 10],
    [7, 3, 10],
    [0, 1, 11],
    [1, 5, 11],
    [5, 4, 11],
    [4, 0, 11],
    [4, 5, 12],
    [5, 7, 12],
    [7, 6, 12],
    [6, 4, 12],
    [0, 2, 13],
    [2, 3, 13],
    [3, 1, 13],
    [1, 0, 13],
];

const OPPOSING_FACET: [usize; 24] = [
    7, 6, 5, 4, 3, 2, 1, 0, 12, 15, 14, 13, 8, 11, 10, 9, 20, 23, 22, 21, 16, 19, 18, 17,
];

/// Structure that manages a data set on a mesh-like geometry
#[derive(Debug)]
pub struct MCMeshDomain<T: Float> {
    /// Global identifier of the domain
    pub domain_gid: usize,
    /// List of domain global identifiers
    pub nbr_domain_gid: Vec<usize>, // maybe not usize
    /// List of ranks
    pub nbr_rank: Vec<usize>, // maybe not usize
    /// List of nodes
    pub node: Vec<MCVector<T>>,
    /// List of connectivity between cells
    pub cell_connectivity: Vec<MCFacetAdjacencyCell>,
    /// List of cells
    pub cell_geometry: Vec<MCFacetGeometryCell<T>>,

    /// Needs replacement
    pub connectivity_facet_storage: BulkStorage<MCFacetAdjacency>, // set capacity totalcells*24
    /// Needs replacement
    pub connectivity_point_storage: BulkStorage<i32>, // set capacity totalcells*14
    /// Needs replacement
    pub geom_facet_storage: BulkStorage<MCGeneralPlane<T>>,
}

impl<T: Float> MCMeshDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition,
        grid: &GlobalFccGrid<T>,
        ddc: &DecompositionObject,
        boundary_condition: &MCSubfacetAdjacencyEvent,
    ) -> Self {
        todo!()
    }
}

/// Structure used to manage a region on a domain
#[derive(Debug)]
pub struct MCDomain<T: Float> {
    // pub domain_index: usize, // unused?
    /// Global domain number
    pub global_domain: usize,
    /// List of cells and their state (See [MCCellState] for more)
    pub cell_state: Vec<MCCellState<T>>,
    /// Needs replacement
    //pub cached_cross_section_storage: BulkStorage<f64>,
    /// Mesh of the domain
    pub mesh: MCMeshDomain<T>,
}

impl<T: Float + FromPrimitive> MCDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition,
        grid: &GlobalFccGrid<T>,
        ddc: &DecompositionObject,
        params: &Parameters,
        //material_database: &MaterialDatabase<T>,
        num_energy_groups: usize,
    ) -> Self {
        let mesh = MCMeshDomain::new(mesh_partition, grid, ddc, &get_boundary_conditions(params));
        let cell_state: Vec<MCCellState<T>> = Vec::with_capacity(mesh.cell_geometry.len());
        let mut mcdomain = MCDomain {
            global_domain: mesh.domain_gid,
            cell_state,
            mesh,
        };

        (0..mcdomain.cell_state.len()).into_iter().for_each(|ii| {
            mcdomain.cell_state[ii].volume = mcdomain.cell_volume(ii);

            //let point: MCVector<T> = cell_position_3dg(&mcdomain, ii);
            //let mat_name = find_material(&params.geometry_params, &point);

            mcdomain.cell_state[ii].total = vec![zero(); num_energy_groups];

            //let num_isos: usize = material_database.mat[mcdomain.cell_state[ii].material].iso.len();
            mcdomain.cell_state[ii].cell_number_density = one();

            let cell_center: MCVector<T> = mcdomain.cell_center(ii);
            mcdomain.cell_state[ii].id = grid.which_cell(&cell_center) * 0x0100000000; // ?
            mcdomain.cell_state[ii].source_tally = 0;
        });
        mcdomain
    }

    /// Clears the cross section cache for future uses.
    pub fn clear_cross_section_cache(&mut self) {
        self.cell_state.iter_mut().for_each(|cs| cs.total.clear())
    }

    /// Returns the coordinates of the center of
    /// the specified cell.
    pub fn cell_center(&self, cell_idx: usize) -> MCVector<T> {
        let cell = &self.mesh.cell_connectivity[cell_idx];
        let node = &self.mesh.node;
        let mut center: MCVector<T> = MCVector::default();

        (0..cell.point.len()).into_iter().for_each(|ii| {
            center += node[cell.point[ii]];
        });
        center /= FromPrimitive::from_usize(cell.point.len()).unwrap();
        center
    }

    /// Computes the volume of the specified cell. Replaces
    /// an isolated function of the original code.
    pub fn cell_volume(&self, cell_idx: usize) -> T {
        let center = self.cell_center(cell_idx);
        let cell = &self.mesh.cell_connectivity[cell_idx];
        let node = &self.mesh.node;

        let mut volume: T = zero();

        (0..cell.facet.len()).into_iter().for_each(|facet_idx| {
            let corners = &cell.facet[facet_idx].point;
            let aa: MCVector<T> = node[corners[0].unwrap()] - center;
            let bb: MCVector<T> = node[corners[1].unwrap()] - center;
            let cc: MCVector<T> = node[corners[2].unwrap()] - center;
            volume = volume + aa.dot(&bb.cross(&cc)).abs();
        });
        volume = volume / FromPrimitive::from_f64(6.0).unwrap();
        volume
    }
}

fn bootstrap_node_map<T: Float>(
    partition: &MeshPartition,
    grid: &GlobalFccGrid<T>,
) -> HashMap<u64, usize> {
    todo!()
}

fn build_cells<T: Float>(
    cell: &[MCFacetAdjacencyCell],
    node_idx_map: &HashMap<u64, usize>,
    nbr_domain: &[usize],
    partition: &MeshPartition,
    grid: &GlobalFccGrid<T>,
    boundary_cond: &MCSubfacetAdjacencyEvent,
) {
    todo!()
}

fn make_facet(location: &MCLocation, node_idx: &[usize], face_info: &FaceInfo) -> MCFacetAdjacency {
    todo!()
}

fn find_material<T: Float>(geometry_params: &[GeometryParameters], rr: &MCVector<T>) -> String {
    todo!()
}

fn get_boundary_conditions(params: &Parameters) -> MCSubfacetAdjacencyEvent {
    todo!()
}
