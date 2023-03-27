use std::iter::Sum;
use std::str::FromStr;
use std::{
    fmt::{Debug, Display, LowerExp},
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use num::{Float, FromPrimitive};

pub trait OpsFloat: AddAssign + SubAssign + MulAssign + DivAssign + Sized {}
pub trait UtilsFloat: Default + Debug + Display + LowerExp {}
pub trait CustomFloat:
    Float + FromPrimitive + OpsFloat + UtilsFloat + FromStr + From<f32> + Sum
{
}

impl OpsFloat for f32 {}
impl UtilsFloat for f32 {}
impl CustomFloat for f32 {}

impl OpsFloat for f64 {}
impl UtilsFloat for f64 {}
impl CustomFloat for f64 {}

/// Modules containing all simulation-related constants.
pub mod sim {
    /// Number of timers, i.e. numbers of section we keep track of.
    pub const N_TIMERS: usize = 6;
}

/// Module containing all physics-related constants.
pub mod physical {
    // The below lines of comments are taken directly from Quicksilver
    // ---
    // The values of all physical constants are taken from:
    // 2006 CODATA which is located on the web at
    // http://physics.nist.gov/cuu/Constants/codata.pdf

    // The units of physical quantities used by the code are:
    //    Mass         -  gram (g)
    //    Length       -  centimeter (cm)
    //    Time         -  second (s)
    //    Energy       -  million electron-volts (MeV) : of a particle
    //    Energy       -  erg (g cm^2/s^2): in some background calculation
    //    Temperature  -  thousand electron-volts (keV)

    pub const NEUTRON_REST_MASS_ENERGY: f64 = 9.395656981095e+2; // MeV
    pub const PI: f64 = std::f64::consts::PI;
    pub const LIGHT_SPEED: f64 = 2.99792458e+10; // cm/s

    pub const TINY_FLOAT: f64 = 1e-13;
    pub const SMALL_FLOAT: f64 = 1e-10;
    pub const HUGE_FLOAT: f64 = 1e75;
}

/// Modules containing all mesh and geometry related constants. The used mesh
/// is made of cells (hexahedron), each divided in 12 sub-cells (tetrahedron).
pub mod mesh {
    /// Number of points per tetrahedron facet.
    pub const N_POINTS_PER_FACET: usize = 3;
    /// Number of facets of a cell facing outward i.e. constituting
    /// a border with another cell.
    pub const N_FACETS_OUT: usize = 24;
    /// Number of points defining a cell.
    pub const N_POINTS_INTERSEC: usize = 14;
    /// Number of faces defining a cell.
    pub const N_FACES: usize = 6;
}
