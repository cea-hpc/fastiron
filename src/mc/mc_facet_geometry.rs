use num::FromPrimitive;

use crate::constants::{physical::TINY_FLOAT, CustomFloat};

use super::mc_vector::MCVector;

/// Structure representing a plane of equation `A*x + B*y + C*z + D = 0`
/// (A,B,C) is normalized.
#[derive(Debug, Clone, Copy, Default)]
pub struct MCGeneralPlane<T: CustomFloat> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

impl<T: CustomFloat> MCGeneralPlane<T> {
    pub fn new(r0: &MCVector<T>, r1: &MCVector<T>, r2: &MCVector<T>) -> Self {
        let one: T = FromPrimitive::from_f64(1.0).unwrap();
        let tiny_f: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();

        let mut a = ((r1.y - r0.y) * (r2.z - r0.z)) - ((r1.z - r0.z) * (r2.y - r0.y));
        let mut b = ((r1.z - r0.z) * (r2.x - r0.x)) - ((r1.x - r0.x) * (r2.z - r0.z));
        let mut c = ((r1.x - r0.x) * (r2.y - r0.y)) - ((r1.y - r0.y) * (r2.x - r0.x));
        let mut d = -one * (a * r0.x + b * r0.y + c * r0.z);

        let mut magnitude: T = (a * a + b * b + c * c).sqrt();

        // if magnitude == 0
        if magnitude < tiny_f {
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

/// List of planes associated with the facet of a cell.
pub type MCFacetGeometryCell<T> = Vec<MCGeneralPlane<T>>;
