use std::collections::HashMap;

use num::{one, zero, Float, FromPrimitive};

use crate::{
    decomposition_object::DecompositionObject,
    global_fcc_grid::GlobalFccGrid,
    mesh_partition::{CellInfo, MeshPartition},
    parameters::Parameters,
};

use super::{
    mc_cell_state::MCCellState,
    mc_facet_adjacency::{MCFacetAdjacency, MCFacetAdjacencyCell, MCSubfacetAdjacencyEvent},
    mc_facet_geometry::{MCFacetGeometryCell, MCGeneralPlane},
    mc_location::MCLocation,
    mc_vector::MCVector,
};

#[derive(Debug, Clone, Copy, Default)]
struct FaceInfo {
    pub event: MCSubfacetAdjacencyEvent,
    pub cell_info: CellInfo,
    pub nbr_idx: Option<usize>,
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
    pub nbr_domain_gid: Vec<usize>,
    /// List of ranks
    pub nbr_rank: Vec<usize>,
    /// List of nodes
    pub node: Vec<MCVector<T>>,
    /// List of connectivity between cells
    pub cell_connectivity: Vec<MCFacetAdjacencyCell>,
    /// List of cells
    pub cell_geometry: Vec<MCFacetGeometryCell<T>>,
}

impl<T: Float + FromPrimitive> MCMeshDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition,
        grid: &GlobalFccGrid<T>,
        ddc: &DecompositionObject,
        boundary_condition: &[MCSubfacetAdjacencyEvent],
    ) -> Self {
        // nbr_domain_gid
        let nbr_domain_gid: Vec<usize> = mesh_partition.nbr_domains.clone();

        // nbr_rank
        let mut nbr_rank: Vec<usize> = Vec::with_capacity(nbr_domain_gid.len());
        (0..nbr_domain_gid.len()).into_iter().for_each(|ii| {
            nbr_rank.push(ddc.rank[nbr_domain_gid[ii]]);
        });

        // cell_connectivity
        let node_idx_map = bootstrap_node_map(mesh_partition, grid);
        let cell_connectivity = build_cells(
            &node_idx_map,
            &nbr_domain_gid,
            mesh_partition,
            grid,
            boundary_condition,
        );

        // node
        let mut node: Vec<MCVector<T>> = vec![MCVector::default(); node_idx_map.len()];
        node_idx_map.iter().for_each(|(node_gid, node_idx)| {
            node[*node_idx] = grid.node_coord_from_idx(*node_gid);
        });

        // cell_geometry
        let mut cell_geometry: Vec<MCFacetGeometryCell<T>> =
            Vec::with_capacity(cell_connectivity.len());
        (0..cell_connectivity.len())
            .into_iter()
            .for_each(|cell_idx| {
                let n_facets = cell_connectivity[cell_idx].facet.len(); // TODO: remove and use const; same in array def
                cell_geometry[cell_idx] = Vec::with_capacity(n_facets); // replace MCFacetGeometryCell vec by array?
                (0..n_facets).into_iter().for_each(|facet_idx| {
                    let r0: MCVector<T> =
                        node[cell_connectivity[cell_idx].facet[facet_idx].point[0].unwrap()];
                    let r1: MCVector<T> =
                        node[cell_connectivity[cell_idx].facet[facet_idx].point[1].unwrap()];
                    let r2: MCVector<T> =
                        node[cell_connectivity[cell_idx].facet[facet_idx].point[2].unwrap()];
                    cell_geometry[cell_idx][facet_idx] = MCGeneralPlane::new(&r0, &r1, &r2);
                });
            });

        Self {
            domain_gid: mesh_partition.domain_gid,
            nbr_domain_gid,
            nbr_rank,
            node,
            cell_connectivity,
            cell_geometry,
        }
    }
}

/// Structure used to manage a domain, i.e. a spatial region of the problem
#[derive(Debug)]
pub struct MCDomain<T: Float> {
    /// Global domain number
    pub global_domain: usize,
    /// List of cells and their state (See [MCCellState] for more)
    pub cell_state: Vec<MCCellState<T>>,
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
    ) -> Self {
        let mesh = MCMeshDomain::new(mesh_partition, grid, ddc, &get_boundary_conditions(params));
        let cell_state: Vec<MCCellState<T>> = Vec::with_capacity(mesh.cell_geometry.len());
        let mut mcdomain = MCDomain {
            global_domain: mesh.domain_gid,
            cell_state,
            mesh,
        };

        let num_energy_groups: usize = params.simulation_params.n_groups;

        (0..mcdomain.cell_state.len()).into_iter().for_each(|ii| {
            mcdomain.cell_state[ii].volume = mcdomain.cell_volume(ii);

            mcdomain.cell_state[ii].total = vec![zero(); num_energy_groups];

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

    /// Computes the volume of the specified cell.
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

/// Need to compare to original code
fn bootstrap_node_map<T: Float + FromPrimitive>(
    partition: &MeshPartition,
    grid: &GlobalFccGrid<T>,
) -> HashMap<usize, usize> {
    let mut node_idx_map: HashMap<usize, usize> = Default::default();

    for (k, v) in &partition.cell_info_map {
        if v.domain_gid.unwrap() != partition.domain_gid {
            continue;
        }
        let node_gids = grid.get_node_gids(*k);
        (0..14).into_iter().for_each(|ii| {
            node_idx_map.insert(node_gids[ii], node_idx_map.len());
        });
    }

    node_idx_map
}

/// Build the cells object of the mesh.
fn build_cells<T: Float + FromPrimitive>(
    node_idx_map: &HashMap<usize, usize>,
    nbr_domain: &[usize],
    partition: &MeshPartition,
    grid: &GlobalFccGrid<T>,
    boundary_cond: &[MCSubfacetAdjacencyEvent],
) -> Vec<MCFacetAdjacencyCell> {
    // nbr_domain_idx[domain_gid] = local_nbr_idx
    let mut nbr_domain_idx: HashMap<usize, Option<usize>> = Default::default();
    (0..nbr_domain.len()).into_iter().for_each(|ii| {
        nbr_domain_idx.insert(nbr_domain[ii], Some(ii));
    });
    nbr_domain_idx.insert(partition.domain_gid, None);

    // return value
    let mut cell: Vec<MCFacetAdjacencyCell> = Vec::with_capacity(partition.cell_info_map.len());

    partition
        .cell_info_map
        .iter()
        .for_each(|(cell_gid, cell_info)| {
            if cell_info.domain_gid != Some(partition.domain_gid) {
                return;
            }
            let mut new_cell = MCFacetAdjacencyCell::default();

            // nodes
            let node_gid = grid.get_node_gids(*cell_gid);
            (0..new_cell.point.len()).into_iter().for_each(|ii| {
                new_cell.point[ii] = node_idx_map[&node_gid[ii]];
            });

            // faces
            let face_nbr = grid.get_face_nbr_gids(*cell_gid);
            let mut face_info = vec![FaceInfo::default(); 6];
            (0..face_nbr.len()).into_iter().for_each(|ii| {
                // faces
                let face_cell_info = partition.cell_info_map[&face_nbr[ii]];
                face_info[ii].cell_info = face_cell_info;
                face_info[ii].nbr_idx = nbr_domain_idx[&face_cell_info.domain_gid.unwrap()];
                if face_nbr[ii] == *cell_gid {
                    face_info[ii].event = boundary_cond[ii];
                } else if face_cell_info.foreman == cell_info.foreman {
                    face_info[ii].event = MCSubfacetAdjacencyEvent::TransitOnProcessor;
                } else {
                    face_info[ii].event = MCSubfacetAdjacencyEvent::TransitOffProcessor;
                }
            });

            // facets
            let mut location = MCLocation {
                domain: cell_info.domain_index,
                cell: cell_info.cell_index,
                facet: None,
            };
            (0..new_cell.facet.len()).into_iter().for_each(|ii| {
                location.facet = Some(ii);
                make_facet(
                    &mut new_cell.facet[ii],
                    &location,
                    &new_cell.point,
                    &face_info,
                );
            });

            cell.push(new_cell);
        });

    cell
}

/// Complete a facet object.
fn make_facet(
    facet: &mut MCFacetAdjacency,
    location: &MCLocation,
    node_idx: &[usize],
    face_info: &[FaceInfo],
) {
    let facet_id = location.facet.unwrap();
    let face_id = facet_id / 4;

    facet.point[0] = Some(node_idx[NODE_INDIRECT[facet_id][0]]);
    facet.point[1] = Some(node_idx[NODE_INDIRECT[facet_id][1]]);
    facet.point[2] = Some(node_idx[NODE_INDIRECT[facet_id][2]]);
    facet.subfacet.event = face_info[face_id].event;
    facet.subfacet.current = *location;
    facet.subfacet.adjacent.domain = face_info[face_id].cell_info.domain_index;
    facet.subfacet.adjacent.cell = face_info[face_id].cell_info.cell_index;
    facet.subfacet.adjacent.facet = Some(OPPOSING_FACET[facet_id]);
    facet.subfacet.neighbor_index = face_info[face_id].nbr_idx;
    facet.subfacet.neighbor_global_domain = face_info[face_id].cell_info.domain_gid;
    facet.subfacet.neighbor_foreman = face_info[face_id].cell_info.foreman;

    match facet.subfacet.event {
        MCSubfacetAdjacencyEvent::BoundaryReflection | MCSubfacetAdjacencyEvent::BoundaryEscape => {
            facet.subfacet.adjacent.facet = facet.subfacet.current.facet;
        }
        _ => (),
    }
}

/// Match the boundary conditions of Parameters to its Enum representation.
fn get_boundary_conditions(params: &Parameters) -> [MCSubfacetAdjacencyEvent; 6] {
    match params.simulation_params.boundary_condition.as_ref() {
        "reflect" => [MCSubfacetAdjacencyEvent::BoundaryReflection; 6],
        "escape" => [MCSubfacetAdjacencyEvent::BoundaryEscape; 6],
        "octant" => [
            MCSubfacetAdjacencyEvent::BoundaryEscape,
            MCSubfacetAdjacencyEvent::BoundaryReflection,
            MCSubfacetAdjacencyEvent::BoundaryEscape,
            MCSubfacetAdjacencyEvent::BoundaryReflection,
            MCSubfacetAdjacencyEvent::BoundaryEscape,
            MCSubfacetAdjacencyEvent::BoundaryReflection,
        ],
        _ => unreachable!(),
    }
}
