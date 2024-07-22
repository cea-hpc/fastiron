//! Hardcoded constants used by the simulation
//!
//! Constants are sorted in submodules according to their nature.
//! Are also included aliases, as well as the custom trait used to
//! introduce a generic floating point type in the code.
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

/// Associated reference value used for compute approximation.
///
/// This is the only trait that should be manually implemented for custom types. The `CustomFloat` trait is then
/// automatically implemented via a blanket implementation.
pub trait CustomReferenceFloat: Float + FromPrimitive {
    /// Threshold upper-value for decimal number.
    const HUGE_FLOAT: Self;
    /// Threshold low-ish-value for decimal number.
    const SMALL_FLOAT: Self;
    /// Threshold lower-value for decimal number.
    const TINY_FLOAT: Self;
}

/// Custom trait for floating point number
pub trait CustomFloat:
    Float
    + CustomReferenceFloat
    + Default
    // conversions
    + FromPrimitive
    + FromStr
    // operations
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Sum
    // display
    + Debug
    + Display
    + LowerExp
    // parallel-safe
    + Send
    + Sync
{
    /// Threshold upper-value for decimal number.
    fn huge_float<T: CustomFloat>() -> T {
        T::from(T::HUGE_FLOAT).unwrap()
    }
    /// Threshold low-ish-value for decimal number.
    fn small_float<T: CustomFloat>() -> T {
        T::from(T::SMALL_FLOAT).unwrap()
    }
    /// Threshold lower-value for decimal number.
    fn tiny_float<T: CustomFloat>() -> T {
        T::from(T::TINY_FLOAT).unwrap()
    }

    fn neutron_mass_energy<T: CustomFloat>() -> T {
        T::from(9.39565e+2).unwrap()
    }

    fn pi<T: CustomFloat>() -> T {
        T::from(std::f32::consts::PI).unwrap()
    }

    fn light_speed<T: CustomFloat>() -> T {
        T::from(2.99792e+10).unwrap()
    }
}

impl CustomReferenceFloat for f32 {
    /// Threshold value for decimal number when using [f32]. May need adjustment.
    const HUGE_FLOAT: Self = 10e35_f32;

    /// Threshold value for decimal number when using [f32]. May need adjustment.
    const SMALL_FLOAT: Self = 1e-10_f32;

    /// Threshold value for decimal number when using [f32]. May need adjustment.
    const TINY_FLOAT: Self = 1e-13_f32;
}

impl CustomReferenceFloat for f64 {
    /// Threshold value for decimal number when using [f64].
    const HUGE_FLOAT: Self = 10e75_f64;

    /// Threshold value for decimal number when using [f64].
    const SMALL_FLOAT: Self = 1e-10;

    /// Threshold value for decimal number when using [f64].
    const TINY_FLOAT: Self = 1e-13;
}

// blanket impl
impl<
        T: Float
            + CustomReferenceFloat
            + Default
            + FromPrimitive
            + FromStr
            + AddAssign
            + SubAssign
            + MulAssign
            + DivAssign
            + Sum
            + Debug
            + Display
            + LowerExp
            + Send
            + Sync,
    > CustomFloat for T
{
}

//===================
// constants modules
//===================

/// Simulation-related constants
///
/// The constants here have no physical grounding and are just related to the nature of
/// the running simulation.
pub mod sim {
    /// Fraction of the target number of particles to spawn at each cycle,
    /// independently of the state of the system.
    pub const SRC_FRACTION: f64 = 0.1;
    /// Number of timers, i.e. numbers of section we keep track of
    pub const N_TIMERS: usize = 6;
    /// Number of particle species
    pub const N_SPECIES: usize = 1;
}
