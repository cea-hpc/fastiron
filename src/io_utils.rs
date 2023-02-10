use clap::Parser;

use crate::parameters::Parameters;

enum InputError {
    BadInputFile,
    BadGeometryBlock,
    BadMaterialBlock,
    BadCrossSectionBlock,
}

#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    /// name of input file
    #[arg(short = 'i', long = "input-file", num_args(1))]
    pub input_file: Option<String>,

    /// name of energy spectrum output file
    #[arg(short = 'e', long = "energy-spectrum", num_args(1))]
    pub energy_spectrum: Option<String>,

    /// name of cross-section output file
    #[arg(short = 'S', long = "cross-sections", num_args(1))]
    pub cross_sections_out: Option<String>,

    /// time step in seconds
    #[arg(short = 'D', long = "dt", num_args(1), allow_negative_numbers(false))]
    pub dt: Option<f64>,

    /// max random mesh node displacement
    #[arg(short = 'f', long = "f-max", num_args(1))]
    pub f_max: Option<f64>,

    /// enable load balancing if present
    #[arg(short = 'l', long = "load-balance", num_args(0))]
    pub load_balance: bool,

    /// enable cycle timers if present
    #[arg(short = 'c', long = "cycle-timers", num_args(0))]
    pub cycle_timers: bool,

    // TO ADD: debug thread level?
    /// x-size of simulation in cm
    #[arg(short = 'X', long = "lx", num_args(1), allow_negative_numbers(false))]
    pub lx: Option<f64>,

    /// y-size of simulation in cm
    #[arg(short = 'Y', long = "ly", num_args(1), allow_negative_numbers(false))]
    pub ly: Option<f64>,

    /// z-size of simulation in cm
    #[arg(short = 'Z', long = "lz", num_args(1), allow_negative_numbers(false))]
    pub lz: Option<f64>,

    /// total number of particules
    #[arg(
        short = 'n',
        long = "n-particles",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub n_particles: Option<u64>,

    /// number of particles in a vault/batch
    #[arg(
        short = 'g',
        long = "batch-size",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub batch_size: Option<u64>,

    /// number of vault/batch to start; sets batch-size automatically if specified
    #[arg(
        short = 'b',
        long = "n-batches",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub n_batches: Option<u64>,

    /// number of steps simulated
    #[arg(
        short = 'N',
        long = "n-steps",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub n_steps: Option<u32>,

    /// number of mesh elements along x
    #[arg(short = 'x', long = "nx", num_args(1), allow_negative_numbers(false))]
    pub nx: Option<u32>,

    /// number of mesh elements along y
    #[arg(short = 'y', long = "ny", num_args(1), allow_negative_numbers(false))]
    pub ny: Option<u32>,

    /// number of mesh elements along z
    #[arg(short = 'z', long = "nz", num_args(1), allow_negative_numbers(false))]
    pub nz: Option<u32>,

    /// random number seed
    #[arg(short = 's', long = "seed", num_args(1), allow_negative_numbers(false))]
    pub seed: Option<u32>, //maybe allow negative values ? need to test QS behavior

    /*
    /// number of MPI ranks along x
    #[arg(short='I', long="x-dom", num_args(1), allow_negative_numbers(false))]
    pub x_dom: Option<u32>,

    /// number of MPI ranks along y
    #[arg(short='J', long="y-dom", num_args(1), allow_negative_numbers(false))]
    pub y_dom: Option<u32>,

    /// number of MPI ranks along z
    #[arg(short='K', long="z-dom", num_args(1), allow_negative_numbers(false))]
    pub z_dom: Option<u32>,
    */
    /// number of balance tally replications
    #[arg(
        short = 'B',
        long = "b-tally",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub balance_tally_replications: Option<u32>,

    /// number of scalar flux tally replications
    #[arg(
        short = 'F',
        long = "f-tally",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub flux_tally_replications: Option<u32>,

    /// number of scalar cell tally replications
    #[arg(
        short = 'C',
        long = "c-tally",
        num_args(1),
        allow_negative_numbers(false)
    )]
    pub cell_tally_replications: Option<u32>,
}

pub fn parse_input_file(filename: String, params: &mut Parameters) {
    //todo!()
}
