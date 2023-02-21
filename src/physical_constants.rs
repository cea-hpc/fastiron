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

// adjustment needed ?
pub const TINY_FLOAT: f64 = 1e-13;
pub const SMALL_FLOAT: f64 = 1e-10;
pub const HUGE_FLOAT: f64 = 1e75;