//! Facet modelling & related computation
//!
//! This module contains the code used for the modelling and computations
//! related to the facets of the mesh.

use super::{mc_location::MCLocation, N_FACETS_OUT, N_POINTS_INTERSEC, N_POINTS_PER_FACET};
use crate::{constants::CustomFloat, data::mc_vector::MCVector};
use num::{one, zero};

//==============
// Spatial plane
//==============

/// Structure representing a plane of equation `a*x + b*y + c*z + d = 0`
///
/// (a, b, c) is the surface normal.
#[derive(Debug, Clone, Copy, Default)]
pub struct MCGeneralPlane<T: CustomFloat> {
    /// x axis coefficient.
    pub a: T,
    /// y axis coefficient.
    pub b: T,
    /// z axis coefficient.
    pub c: T,
    /// offset coefficient.
    pub d: T,
}

impl<T: CustomFloat> MCGeneralPlane<T> {
    /// Constructor. This creates an object corresponding to the plane formed by the
    /// three points passed as arguments.
    pub fn new(points: &[MCVector<T>]) -> Self {
        let one: T = one();
        assert_eq!(points.len(), 3);
        let (r0, r1, r2) = (points[0], points[1], points[2]);

        let mut a = ((r1.y - r0.y) * (r2.z - r0.z)) - ((r1.z - r0.z) * (r2.y - r0.y));
        let mut b = ((r1.z - r0.z) * (r2.x - r0.x)) - ((r1.x - r0.x) * (r2.z - r0.z));
        let mut c = ((r1.x - r0.x) * (r2.y - r0.y)) - ((r1.y - r0.y) * (r2.x - r0.x));
        let mut d = -one * (a * r0.x + b * r0.y + c * r0.z);

        let magnitude: T = (a * a + b * b + c * c).sqrt();

        // if magnitude == 0
        if magnitude == zero() {
            a = one;
            // early return should prevent fp error
            return Self { a, b, c, d };
        }
        // normalize
        a /= magnitude;
        b /= magnitude;
        c /= magnitude;
        d /= magnitude;

        Self { a, b, c, d }
    }
}

/// List of planes associated with the outward-facing facets of a cell.
pub type MCFacetGeometryCell<T> = [MCGeneralPlane<T>; N_FACETS_OUT];

//==============
// Nearest facet
//==============

/// Structure used to represent the nearest facet to a particle,
/// holding relevant data for computation.
#[derive(Debug, Clone, Copy)]
pub struct MCNearestFacet<T: CustomFloat> {
    /// Index of the facet the particle is the closest to.
    pub facet: usize,
    /// Distance between facet and particle.
    pub distance_to_facet: T,
    /// Dot product between facet and direction vector.
    pub dot_product: T,
}

impl<T: CustomFloat> Default for MCNearestFacet<T> {
    fn default() -> Self {
        Self {
            facet: 0,
            distance_to_facet: T::huge_float(),
            dot_product: zero(),
        }
    }
}

//================
// Facet adjacency
//================

/// Enum used to categorize the event a particle
/// undergo when reaching a given facet.
///
/// This value essentially depends on the nature of what is on the other side.
#[derive(Debug, Clone, Copy, Default)]
pub enum MCSubfacetAdjacencyEvent {
    /// Default value. This will generate error at runtime if not initialized
    /// correctly.
    #[default]
    AdjacencyUndefined = 0,
    /// Value corresponding to an escape event. The facet is located at the
    /// edge of the problem and the behavior is set to allow escape.
    BoundaryEscape,
    /// Value corresponding to a reflection event. The facet is located at the
    /// edge of the problem and the behavior of the facet is set to reflect.
    BoundaryReflection,
    /// Value corresponding to an intra-problem crossing event. The neighboring
    /// facet belongs to a cell managed by the same processor.
    TransitOnProcessor,
    /// Value corresponding to an intra-problem crossing event. The neighboring
    /// facet belongs to a cell managed by a different processors.
    TransitOffProcessor,
}

/// Sub-structure for adjacent facet representation.
///
/// This structure is _oriented_, i.e. there is a current cell and a neighbor
/// cell.
#[derive(Debug, Default, Clone, Copy)]
pub struct SubfacetAdjacency {
    /// Event associated with the facet junction.
    pub event: MCSubfacetAdjacencyEvent,
    /// Current location.
    pub current: MCLocation,
    /// Neighboring location.
    pub adjacent: MCLocation,
    /// Neighbor index.
    pub neighbor_index: Option<usize>,
    /// Neighbor domain global identifier.
    pub neighbor_global_domain: Option<usize>,
    /// Neighbor foreman.
    pub neighbor_foreman: Option<usize>,
}

/// Structure for adjacent facet representation.
#[derive(Debug, Default, Clone, Copy)]
pub struct MCFacetAdjacency {
    /// Adjacency data.
    pub subfacet: SubfacetAdjacency,
    /// Point indexes for this facet. The points are defined in a private constant
    /// in the [mc_domain][super::mc_domain] module.
    pub point: [usize; N_POINTS_PER_FACET],
}

/// Structure encompassing all adjacent facet to a cell.
#[derive(Debug, Default)]
pub struct MCFacetAdjacencyCell {
    pub facet: [MCFacetAdjacency; N_FACETS_OUT],
    pub point: [usize; N_POINTS_INTERSEC],
}

impl MCFacetAdjacencyCell {
    pub fn get_planes<T: CustomFloat>(
        &self,
        node: &[MCVector<T>],
    ) -> [MCGeneralPlane<T>; N_FACETS_OUT] {
        self.facet.map(|facet_adjacency| {
            let points = facet_adjacency.point.map(|idx| node[idx]);
            MCGeneralPlane::new(&points)
        })
    }
}
