//! Facet modelling & related computation

use num::{zero, FromPrimitive};

use crate::{
    constants::{mesh::N_FACETS_OUT, CustomFloat},
    data::mc_vector::MCVector,
};

/// Structure used to represent the distance to a given facet.
///
/// This structure is used in order to group information used when assessing the
/// next event for a particle and, in the case of a facet crossing, going through
/// with it.
#[derive(Debug, Clone, Copy, Default)]
pub struct MCDistanceToFacet<T: CustomFloat> {
    /// Distance to the given facet in cm.
    pub distance: T,
    /// Index of the given facet.
    pub facet: usize,
    /// Index of the given sub-facet.
    pub subfacet: usize,
}

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
    pub fn new(r0: &MCVector<T>, r1: &MCVector<T>, r2: &MCVector<T>) -> Self {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();

        let mut a = ((r1.y - r0.y) * (r2.z - r0.z)) - ((r1.z - r0.z) * (r2.y - r0.y));
        let mut b = ((r1.z - r0.z) * (r2.x - r0.x)) - ((r1.x - r0.x) * (r2.z - r0.z));
        let mut c = ((r1.x - r0.x) * (r2.y - r0.y)) - ((r1.y - r0.y) * (r2.x - r0.x));
        let mut d = -one * (a * r0.x + b * r0.y + c * r0.z);

        let mut magnitude: T = (a * a + b * b + c * c).sqrt();

        // if magnitude == 0
        if magnitude == zero() {
            a = one;
            magnitude = one;
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
