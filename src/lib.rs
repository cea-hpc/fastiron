//! A Rust port of the Monte-Carlo particle transport proxy-app [Quicksilver][1].
//!
//! Fastiron mimics Monte-Carlo particle transport code to study their behavior on
//! various hardware architectures. The aim of this port is to evaluate Rust's
//! capabilities and performances in the context of parallel programming and
//! scaling problem size.
//!
//! # Quickstart
//!
//! # Example
//!
//! # Useful links
//! - Fastiron [GitHub repository][2]
//! - Quicksilver [GitHub repository][1]
//!
//! [1]: https://github.com/LLNL/Quicksilver
//! [2]: https://github.com/cea-hpc/fastiron

/// Hardcoded constants used by the simulation
pub mod constants;
/// Data structures
pub mod data;
/// Mesh & modelling-related structures
pub mod geometry;
/// Initialization code for the problem
pub mod init_mc;
/// Super-structure used to store the problem's data
pub mod montecarlo;
/// Parameters-managing structure
pub mod parameters;
/// Particle-related code
pub mod particles;
/// Computation & simulation-related code
pub mod simulation;
/// Utilities used both to setup and run the simulation.
pub mod utils;
