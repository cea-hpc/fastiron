//! A Rust port of the Monte-Carlo particle transport proxy-app [Quicksilver][1].
//!
//! Fastiron mimics Monte-Carlo particle transport code to study their behavior on
//! various hardware architectures. The aim of this port is to evaluate Rust's
//! capabilities and performances in the context of parallel programming and
//! scaling problem size.
//!
//! # Quickstart
//!
//! After cloning the [repository][2], fastiron can be executed like any other Rust
//! program using cargo. Run the following:
//! ```shell
//! $ cargo build --release
//! $ cargo run --bin=fastiron
//! ```
//!
//! And see the CLI's usage:
//! ```shell
//! $ cargo run --bin=fastiron
//!
//! Fastiron, a Rust port of the Quicksilver proxy-app
//!
//! Usage: fastiron [OPTIONS]
//!
//! Options:
//!   -i, --input-file <INPUT_FILE>
//!           name of input file
//!   -e, --energy-spectrum <ENERGY_SPECTRUM>
//!           name of energy spectrum output file
//!   -S, --cross-sections <CROSS_SECTIONS_OUT>
//!           name of cross-section output file
//!   -D, --dt <DT>
//!           time step in seconds
//!   -f, --f-max <F_MAX>
//!           max random mesh node displacement
//!   -l, --load-balance
//!           enable load balancing if present
//!   -c, --cycle-timers
//!           enable cycle timers if present
//!   -t, --debug-threads
//!           enable thread debugging if present
//!   -X, --lx <LX>
//!           x-size of simulation in cm
//!   -Y, --ly <LY>
//!           y-size of simulation in cm
//!   -Z, --lz <LZ>
//!           z-size of simulation in cm
//!   -n, --n-particles <N_PARTICLES>
//!           total number of particules
//!   -g, --batch-size <BATCH_SIZE>
//!           number of particles in a vault/batch
//!   -b, --n-batches <N_BATCHES>
//!           number of vault/batch to start; sets batch-size automatically if specified
//!   -N, --n-steps <N_STEPS>
//!           number of steps simulated
//!   -x, --nx <NX>
//!           number of mesh elements along x
//!   -y, --ny <NY>
//!           number of mesh elements along y
//!   -z, --nz <NZ>
//!           number of mesh elements along z
//!   -s, --seed <SEED>
//!           random number seed
//!   -B, --b-tally <BALANCE_TALLY_REPLICATIONS>
//!           number of balance tally replications
//!   -F, --f-tally <FLUX_TALLY_REPLICATIONS>
//!           number of scalar flux tally replications
//!   -C, --c-tally <CELL_TALLY_REPLICATIONS>
//!           number of scalar cell tally replications
//!   -h, --help
//!          Print help
//!   -V, --version
//!           Print version
//!
//! ```
//!
//! # Example
//!
//! You can run one of the examples available using its input file. Note that the
//! parameters specified in the file take priority over the one specified as arguments:
//!
//! ```shell
//! $ cargo run --bin=fastiron --release -- -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -e energy -S section -n 10000
//! ```
//!
//! Fastiron will print the parameters and run the simulation. Two reports will
//! be printed at run-time and, if file names are specified, two files will be
//! created. These four outputs contain data such as event counts, timers value,
//! or final state of the system. To see more about these reports:
//! - link to tallies print summary
//! - link to cycle timers report
//! - link to energy spectrum
//! - link to cross section?
//!
//! # Useful Links
//!
//! - Fastiron [GitHub repository][2]
//! - Quicksilver [GitHub repository][1]
//!
//! [1]: https://github.com/LLNL/Quicksilver
//! [2]: https://github.com/cea-hpc/fastiron

pub mod constants;
pub mod data;
/// Mesh & modelling-related structures
pub mod geometry;
/// Initialization code for the problem
pub mod init_mc;
pub mod montecarlo;
pub mod parameters;
/// Particle-related code
pub mod particles;
pub mod simulation;
/// Utilities used both to setup and run the simulation.
pub mod utils;
