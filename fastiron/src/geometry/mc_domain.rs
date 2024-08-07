//! Code used for the domain separation model
//!
//!

use num::{one, FromPrimitive};
use rustc_hash::FxHashMap;

use crate::{
    constants::CustomFloat,
    data::{material_database::MaterialDatabase, mc_vector::MCVector},
    parameters::{GeometryParameters, Parameters, Shape},
    simulation::mct::cell_position_3dg,
    utils::decomposition_object::DecompositionObject,
};

use super::{
    facets::MCFacetGeometryCell,
    facets::{MCFacetAdjacency, MCFacetAdjacencyCell, MCSubfacetAdjacencyEvent},
    global_fcc_grid::GlobalFccGrid,
    mc_cell_state::MCCellState,
    mc_location::MCLocation,
    mesh_partition::{CellInfo, MeshPartition},
    N_FACES, N_FACETS_OUT, N_POINTS_INTERSEC, N_POINTS_PER_FACET,
};

/// Structure used to hold information related a cell's face.
#[derive(Debug, Clone, Copy, Default)]
pub struct FaceInfo {
    /// Event associated to the crossing of the face.
    pub event: MCSubfacetAdjacencyEvent,
    /// Current cell's data.
    pub cell_info: CellInfo,
    /// Index of the domain of the neighboring cell.
    pub nbr_idx: Option<usize>,
}

const NODE_INDIRECT: [[usize; N_POINTS_PER_FACET]; N_FACETS_OUT] = [
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

const OPPOSING_FACET: [usize; N_FACETS_OUT] = [
    7, 6, 5, 4, 3, 2, 1, 0, 12, 15, 14, 13, 8, 11, 10, 9, 20, 23, 22, 21, 16, 19, 18, 17,
];

/// Structure that manages a data set on a mesh-like geometry.
#[derive(Debug, Default)]
pub struct MCMeshDomain<T: CustomFloat> {
    /// Global identifier associated of the domain.
    pub domain_gid: usize,
    /// Global identifiers of the neighboring domains.
    pub nbr_domain_gid: Vec<usize>,
    /// Ranks of the neighboring domains.
    pub nbr_rank: Vec<usize>,
    /// List of nodes defining the mesh's geometry.
    pub node: Vec<MCVector<T>>,
    /// List of connectivity between cells of the mesh.
    pub cell_connectivity: Vec<MCFacetAdjacencyCell>,
    /// List of the cells of the mesh.
    pub cell_geometry: Vec<MCFacetGeometryCell<T>>,
}

impl<T: CustomFloat> MCMeshDomain<T> {
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
        let nbr_rank: Vec<usize> = nbr_domain_gid
            .iter()
            .map(|domain_gid| ddc.rank[*domain_gid])
            .collect();

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
        let cell_geometry: Vec<MCFacetGeometryCell<T>> = cell_connectivity
            .iter()
            .map(|facet_adjacency_cell| facet_adjacency_cell.get_planes(&node))
            .collect();

        Self {
            domain_gid: mesh_partition.domain_gid,
            nbr_domain_gid,
            nbr_rank,
            node,
            cell_connectivity,
            cell_geometry,
        }
    }

    pub fn get_facet_coords(
        &self,
        cell_idx: usize,
        facet_idx: usize,
    ) -> [MCVector<T>; N_POINTS_PER_FACET] {
        let points = &self.cell_connectivity[cell_idx].facet[facet_idx].point;
        [
            self.node[points[0]],
            self.node[points[1]],
            self.node[points[2]],
        ]
    }
}

/// Structure used to manage a domain, i.e. a spatial region of the problem.
#[derive(Debug, Default)]
pub struct MCDomain<T: CustomFloat> {
    /// Global domain identifier.
    pub global_domain: usize,
    /// List of cells and their state. See [MCCellState] for more information.
    pub cell_state: Vec<MCCellState<T>>,
    /// Mesh of the domain.
    pub mesh: MCMeshDomain<T>,
}

impl<T: CustomFloat> MCDomain<T> {
    /// Constructor.
    pub fn new(
        mesh_partition: &MeshPartition,
        grid: &GlobalFccGrid<T>,
        ddc: &DecompositionObject,
        params: &Parameters<T>,
        mat_db: &MaterialDatabase<T>,
    ) -> Self {
        let mesh = MCMeshDomain::new(mesh_partition, grid, ddc, &get_boundary_conditions(params));
        let cell_state: Vec<MCCellState<T>> = (0..mesh.cell_geometry.len())
            .map(|cell_idx| {
                let rr = cell_position_3dg(&mesh, cell_idx);
                let mat_name = Self::find_material(&params.geometry_params, &rr);
                let cell_center: MCVector<T> = Self::cell_center(&mesh, cell_idx);
                MCCellState {
                    material: mat_db.find_material(&mat_name).unwrap(),
                    volume: Self::cell_volume(&mesh, cell_idx),
                    cell_number_density: one(),
                    id: grid.which_cell(&cell_center) * 0x0100000000,
                    source_tally: 0,
                }
            })
            .collect();

        MCDomain {
            global_domain: mesh.domain_gid,
            cell_state,
            mesh,
        }
    }

    fn find_material(geometry_params: &[GeometryParameters<T>], rr: &MCVector<T>) -> String {
        let mut mat_name = String::default();

        geometry_params.iter().rev().for_each(|geom| {
            if Self::is_inside(geom, rr) {
                // cant return directly because of the behavior of original function
                mat_name.clone_from(&geom.material_name);
            }
        });

        mat_name
    }

    fn is_inside(geom: &GeometryParameters<T>, rr: &MCVector<T>) -> bool {
        match geom.shape {
            Shape::Brick => {
                let in_x = (rr.x >= geom.x_min) & (rr.x <= geom.x_max);
                let in_y = (rr.y >= geom.y_min) & (rr.y <= geom.y_max);
                let in_z = (rr.z >= geom.z_min) & (rr.z <= geom.z_max);
                in_x & in_y & in_z
            }
            Shape::Sphere => {
                let center: MCVector<T> = MCVector {
                    x: geom.x_center,
                    y: geom.y_center,
                    z: geom.z_center,
                };
                (*rr - center).length() <= geom.radius
            }
            Shape::Undefined => unreachable!(),
        }
    }

    /// Returns the coordinates of the center of
    /// the specified cell.
    fn cell_center(mesh: &MCMeshDomain<T>, cell_idx: usize) -> MCVector<T> {
        let cell = &mesh.cell_connectivity[cell_idx];
        let node = &mesh.node;
        let mut center: MCVector<T> = (0..N_POINTS_INTERSEC).map(|ii| node[cell.point[ii]]).sum();
        center /= FromPrimitive::from_usize(cell.point.len()).unwrap();
        center
    }

    /// Computes the volume of the specified cell.
    fn cell_volume(mesh: &MCMeshDomain<T>, cell_idx: usize) -> T {
        let center = Self::cell_center(mesh, cell_idx);
        let cell = &mesh.cell_connectivity[cell_idx];
        let node = &mesh.node;

        let mut volume: T = cell
            .facet
            .iter()
            .map(|facet_adj| {
                let corners = &facet_adj.point;
                let aa: MCVector<T> = node[corners[0]] - center;
                let bb: MCVector<T> = node[corners[1]] - center;
                let cc: MCVector<T> = node[corners[2]] - center;
                aa.dot(&bb.cross(&cc)).abs()
            })
            .sum();
        volume /= FromPrimitive::from_f64(6.0).unwrap();
        volume
    }
}

fn bootstrap_node_map<T: CustomFloat>(
    partition: &MeshPartition,
    grid: &GlobalFccGrid<T>,
) -> FxHashMap<usize, usize> {
    // res
    let mut node_idx_map: FxHashMap<usize, usize> = Default::default();
    // intermediate struct
    let mut face_centers: FxHashMap<usize, usize> = Default::default();

    for (k, v) in &partition.cell_info_map {
        // only process partition of our domain
        if v.domain_gid.unwrap() != partition.domain_gid {
            continue;
        }
        // process
        let node_gids = grid.get_node_gids(*k);
        // corners first
        (0..8).for_each(|ii| {
            let len = node_idx_map.len();
            node_idx_map.entry(node_gids[ii]).or_insert_with(|| len);
        });
        // faces later
        (8..14).for_each(|ii| {
            let len = face_centers.len();
            face_centers.entry(node_gids[ii]).or_insert_with(|| len);
        });
    }

    face_centers.values_mut().for_each(|val| {
        *val += node_idx_map.len();
    });

    node_idx_map.extend(face_centers.iter());

    node_idx_map
}

/// Build the cells object of the mesh.
fn build_cells<T: CustomFloat>(
    node_idx_map: &FxHashMap<usize, usize>,
    nbr_domain: &[usize],
    partition: &MeshPartition,
    grid: &GlobalFccGrid<T>,
    boundary_cond: &[MCSubfacetAdjacencyEvent],
) -> Vec<MCFacetAdjacencyCell> {
    // nbr_domain_idx[domain_gid] = local_nbr_idx
    let mut nbr_domain_idx: FxHashMap<usize, Option<usize>> = Default::default();
    (0..nbr_domain.len()).for_each(|ii| {
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
            let node_gid: [usize; N_POINTS_INTERSEC] = grid.get_node_gids(*cell_gid);
            (0..N_POINTS_INTERSEC).for_each(|ii| {
                new_cell.point[ii] = node_idx_map[&node_gid[ii]];
            });

            // faces
            let face_nbr: [usize; N_FACES] = grid.get_face_nbr_gids(*cell_gid);
            let mut face_info = vec![FaceInfo::default(); N_FACES];
            (0..N_FACES).for_each(|ii| {
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
            (0..N_FACETS_OUT).for_each(|ii| {
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

    facet.point[0] = node_idx[NODE_INDIRECT[facet_id][0]];
    facet.point[1] = node_idx[NODE_INDIRECT[facet_id][1]];
    facet.point[2] = node_idx[NODE_INDIRECT[facet_id][2]];
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
fn get_boundary_conditions<T: CustomFloat>(
    params: &Parameters<T>,
) -> [MCSubfacetAdjacencyEvent; 6] {
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

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_material() {
        // Sphere centered at (2.0, 2.0, 2.0), radius 1.0
        let geom_a = GeometryParameters {
            material_name: String::from("mat_a"),
            shape: Shape::Sphere,
            radius: 1.0,
            x_center: 2.0,
            y_center: 2.0,
            z_center: 2.0,
            ..Default::default()
        };
        let geom_b = GeometryParameters {
            material_name: String::from("mat_b"),
            shape: Shape::Brick,
            x_min: 0.0,
            y_min: 0.0,
            z_min: 0.0,
            x_max: 4.0,
            y_max: 2.0,
            z_max: 1.0,
            ..Default::default()
        };
        let geom_c = GeometryParameters {
            material_name: String::from("mat_c"),
            shape: Shape::Sphere,
            radius: 2.0,
            x_center: 6.0,
            y_center: 2.0,
            z_center: 2.0,
            ..Default::default()
        };
        let geom_d = GeometryParameters {
            material_name: String::from("mat_d"),
            shape: Shape::Brick,
            x_min: 0.0,
            y_min: 2.0,
            z_min: 0.0,
            x_max: 4.0,
            y_max: 4.0,
            z_max: 1.0,
            ..Default::default()
        };

        // is_inside
        let r1 = MCVector {
            x: 2.0,
            y: 1.0,
            z: 0.5,
        }; // in brick b
        assert!(MCDomain::is_inside(&geom_b, &r1));

        let r2 = MCVector {
            x: 2.0,
            y: 2.1,
            z: 0.5,
        }; // out brick b, in brick d
        assert!(!MCDomain::is_inside(&geom_b, &r2));

        let r3 = MCVector {
            x: 1.5,
            y: 2.0,
            z: 2.5,
        }; // in sphere a
        assert!(MCDomain::is_inside(&geom_a, &r3));

        let r4 = MCVector {
            x: 2.0,
            y: 2.0,
            z: 3.1,
        }; // out sphere a
        assert!(!MCDomain::is_inside(&geom_a, &r4));

        let r5 = MCVector {
            x: 2.0,
            y: 2.0,
            z: 1.0,
        }; // in both a, b (single common point)
        assert!(MCDomain::is_inside(&geom_a, &r5));
        assert!(MCDomain::is_inside(&geom_b, &r5));

        let r6 = MCVector {
            x: 5.0,
            y: 2.0,
            z: 2.0,
        }; // out both a, b, in c
        assert!(!MCDomain::is_inside(&geom_b, &r6));
        assert!(!MCDomain::is_inside(&geom_b, &r6));

        // find_material
        let geoms = vec![geom_a, geom_b, geom_c, geom_d];
        assert_eq!(MCDomain::find_material(&geoms, &r1), String::from("mat_b"));
        assert_eq!(MCDomain::find_material(&geoms, &r2), String::from("mat_d"));
        assert_eq!(MCDomain::find_material(&geoms, &r3), String::from("mat_a"));
        assert_eq!(MCDomain::find_material(&geoms, &r5), String::from("mat_a")); // first of the list takes priority
        assert_eq!(MCDomain::find_material(&geoms, &r6), String::from("mat_c"));
    }
}
