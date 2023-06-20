//! Statistical Analysis of [Fastiron]
//!
//! This tool is documented to ease future modifications of the code.
//! The program can be run like any other cargo projects:
//!
//! ```shell
//! cargo run --release --bin=fastiron-stats
//! ```
//!
//! The executable does not take any argument through the command line. Instead,
//! prompts are used to fetch the necessary inputs. There are currently three
//! supported computations:
//!
//! - **Version comparison**: Simple relative differences computation between two timer
//!   reports. This is useful to have numbers quickly and easily. The presented
//!   results are percentages and **positiveness / negativeness have meaning**.
//! - **Correlation study**: Computes correlation coefficients between tallied events
//!   and section lengths. The results are formatted in a `.dat` file that can be
//!   visualized using the corresponding `gnuplot` scripts.
//! - **Scaling study**: Compiles data from a collection of timer to a `.dat` file
//!   that can be used to plot section lengths depending on total number of particles
//!   using a `gnuplot` script. Currently, only arithmetic progression is supported
//!   for the scaling number of particle. Geometric progression support will be added
//!   in the future, as well as a specific script to plot this data using a logarithmic
//!   scale.
//!
//! The user will be prompted first on which computations he wishes to do, only then
//! will specific data be requested for processing. Additionally, the user can
//! automatically provide inputs by redirecting a file to the program:
//!
//! ```shell
//! cargo run --release --bin=fastiron-stats -- < sample_data/auto_input_example
//! ```
//!
//! The control flow being simple, it is easy to pre-write a set of answer to achieve
//! the desired results.
//!
//! [Fastiron]: https://github.com/cea-hpc/fastiron

pub mod io;
pub mod processing;
pub mod structures;
