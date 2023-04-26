//! Hardcoded constants used by the simulation
//!
//! Constants are sorted in sub-modules according to their nature.
//! Are also included aliases, as well as the custom trait used to
//! introduce a generic floatting point type in the code.

use std::iter::Sum;
use std::str::FromStr;
use std::{
    fmt::{Debug, Display, LowerExp},
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use num::{Float, FromPrimitive};

//=======================
// custom traits & types
//=======================

// some alias for readability

/// Custom alias for readability.
pub type Tuple3 = (usize, usize, usize);
/// Custom alias for readability.
pub type Tuple4 = (usize, usize, usize, usize);

// generic float type

/// Custom trait for floatting point number
pub trait OpsFloat: AddAssign + SubAssign + MulAssign + DivAssign + Sized {}
/// Custom trait for floatting point number
pub trait UtilsFloat: Default + Debug + Display + LowerExp + FromStr + From<f32> + Sum {}
/// Custom super-trait for floatting point number
pub trait CustomFloat: Float + FromPrimitive + OpsFloat + UtilsFloat {
    // floating ref

    /// Threshold upper-value for decimal number.
    fn huge_float<T: CustomFloat>() -> T;
    /// Threshold low-ish-value for decimal number.
    fn small_float<T: CustomFloat>() -> T;
    /// Threshold lower-value for decimal number.
    fn tiny_float<T: CustomFloat>() -> T;

    // physical constants

    /// Neutron mass energy equivalent (MeV)
    fn neutron_mass_energy<T: CustomFloat>() -> T;
    /// [Pick your definition][3]
    ///
    /// [3]: https://en.wikipedia.org/wiki/Pi
    fn pi<T: CustomFloat>() -> T;
    /// Speed of light (cm/s)
    fn light_speed<T: CustomFloat>() -> T;
}

impl OpsFloat for f32 {}
impl UtilsFloat for f32 {}
impl CustomFloat for f32 {
    /// Threshold value for decimal number when using [f32]. May need adjustment.
    fn huge_float<T: CustomFloat>() -> T {
        FromPrimitive::from_f32(10e35_f32).unwrap()
    }

    /// Threshold value for decimal number when using [f32]. May need adjustment.
    fn small_float<T: CustomFloat>() -> T {
        FromPrimitive::from_f32(1e-10_f32).unwrap()
    }

    /// Threshold value for decimal number when using [f32]. May need adjustment.
    fn tiny_float<T: CustomFloat>() -> T {
        FromPrimitive::from_f32(1e-13_f32).unwrap()
    }

    fn neutron_mass_energy<T: CustomFloat>() -> T {
        FromPrimitive::from_f32(9.39565e+2).unwrap()
    }

    fn pi<T: CustomFloat>() -> T {
        FromPrimitive::from_f32(std::f32::consts::PI).unwrap()
    }

    fn light_speed<T: CustomFloat>() -> T {
        FromPrimitive::from_f32(2.99792e+10).unwrap()
    }
}

impl OpsFloat for f64 {}
impl UtilsFloat for f64 {}
impl CustomFloat for f64 {
    /// Threshold value for decimal number when using [f64].
    fn huge_float<T: CustomFloat>() -> T {
        FromPrimitive::from_f64(10e75_f64).unwrap()
    }

    /// Threshold value for decimal number when using [f64].
    fn small_float<T: CustomFloat>() -> T {
        FromPrimitive::from_f64(1e-10).unwrap()
    }

    /// Threshold value for decimal number when using [f64].
    fn tiny_float<T: CustomFloat>() -> T {
        FromPrimitive::from_f64(1e-13).unwrap()
    }

    fn neutron_mass_energy<T: CustomFloat>() -> T {
        FromPrimitive::from_f64(9.395656981095e+2).unwrap()
    }

    fn pi<T: CustomFloat>() -> T {
        FromPrimitive::from_f64(std::f64::consts::PI).unwrap()
    }

    fn light_speed<T: CustomFloat>() -> T {
        FromPrimitive::from_f64(2.99792458e+10).unwrap()
    }
}

//===================
// constants modules
//===================

/// Simulation-related constants
pub mod sim {
    //!
    //! The constants here have no physical grounding and are just related to
    //! the nature of the running simulation.

    /// Fraction of the target number of particles to spawn at each cycle,
    /// independently of the state of the system.
    pub const SRC_FRACTION: f64 = 0.1;
    /// Number of timers, i.e. numbers of section we keep track of
    pub const N_TIMERS: usize = 6;
    /// Number of particle species
    pub const N_SPECIES: usize = 1;
}

/// Physics-related constants
pub mod physical {
    //!
    //! The below text is taken directly from a Quicksilver [source file][1]:
    //!
    //! The values of all physical constants are taken from
    //! 2006 CODATA which is located [here][2].
    //!
    //! The units of physical quantities used by the code are:
    //!
    //! |   Quantity     |  Unit
    //! |----------------|---------------------------------------------------
    //! |   Mass         |  gram (g)
    //! |   Length       |  centimeter (cm)
    //! |   Time         |  second (s)
    //! |   Energy       |  million electron-volts (MeV) : of a particle
    //! |   Energy       |  erg (g cm^2/s^2): in some background calculation
    //! |   Temperature  |  thousand electron-volts (keV)
    //!
    //! [1]: https://github.com/LLNL/Quicksilver/blob/master/src/PhysicalConstants.cc
    //! [2]: http://physics.nist.gov/cuu/Constants/codata.pdf

    /// Neutron rest energy (MeV)
    pub const NEUTRON_REST_MASS_ENERGY: f64 = 9.395656981095e+2;
    /// [Pick your definition][3]
    ///
    /// [3]: https://en.wikipedia.org/wiki/Pi
    pub const PI: f64 = std::f64::consts::PI;
    /// Light speed (cm/s)
    pub const LIGHT_SPEED: f64 = 2.99792458e+10;
}
