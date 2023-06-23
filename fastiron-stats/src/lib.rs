//! Statistical Analysis of [Fastiron]
//!
//! This tool is documented to ease future modifications of the code.
//! The program can be run like any other cargo projects:
//!
//! ```shell
//! cargo run --release --bin=fastiron-stats
//! ```
//!
//! This will print out the command line help message:
//!
//! ```shell
//! fastiron-stats, a profiling companion for Fastiron
//!
//! Usage: fastiron-stats [OPTIONS]
//!
//! Options:
//!   -T, --timers-comparison <COMPARISON> <COMPARISON>
//!           name of the two timers report files to compare - old report first, new report second
//!   -E, --event-correlation <CORRELATION>
//!           name of the tallies file to analyze
//!   -W, --weak-scaling <WEAK_SCALING_ROOT>
//!           root path of timers file for weak scaling data
//!   -S, --strong-scaling <STRONG_SCALING_ROOT>
//!           root path of timers file for strong scaling data
//!   -t, --thread_init_n <T_INIT>
//!           starting number of threads for scaling data
//!   -i, --thread_iter_n <T_ITER>
//!           starting number of threads for scaling data
//!   -f, --thread_step_factor <T_FACTOR>
//!           starting number of threads for scaling data
//!   -p, --plot
//!           if present, plot the results of all computed metrics
//!   -h, --help
//!           Print help
//!   -V, --version
//!           Print version
//! ```
//!
//! [Fastiron]: https://github.com/cea-hpc/fastiron

pub mod command_line;
pub mod structures;
