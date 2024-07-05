//! Mesh & modelling-related structures$
//!
//! This module contains code related to the mesh structure used to simulate the problem.
//! This includes the mesh itself as well as objects used for navigating it. The
//! characteristics of the mesh are defined as constants.
//!
//! The used mesh is made of cells (hexahedron), each divided in 12 sub-cells (tetrahedron).
//! TODO: insert an image?

//=================
// Internal modules
//=================

pub mod facets;
pub mod global_fcc_grid;
pub mod grid_assignment_object;
pub mod mc_cell_state;
pub mod mc_domain;
pub mod mc_location;
pub mod mesh_partition;

//===============
// Mesh constants
//===============

use crate::constants::Tuple4;

/// Number of points per tetrahedron facet.
pub const N_POINTS_PER_FACET: usize = 3;
/// Number of facets of a cell facing outward i.e. constituting
/// a border with another cell.
pub const N_FACETS_OUT: usize = 24;
/// Number of points defining a cell.
pub const N_POINTS_INTERSEC: usize = 14;
/// Offsets of the intersection points of a cell.
pub const CORNER_OFFSET: [Tuple4; N_POINTS_INTERSEC] = [
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
/// Number of faces defining a cell.
pub const N_FACES: usize = 6;
/// Offsets of the faces of a cell.
pub const FACE_OFFSET: [(i32, i32, i32); N_FACES] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];
