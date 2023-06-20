use clap::Parser;

/// fastiron-stats, a profiling companion for Fastiron
#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    /// name of input file
    #[arg(short = 'i', long = "input-file", num_args(1))]
    pub input_file: Option<String>,

    /// root path of timers file for weak scaling data
    #[arg(short = 'W', long = "weak-scaling", num_args(1))]
    pub weak_scaling_root: Option<String>,

    /// root path of timers file for strong scaling data
    #[arg(short = 'S', long = "strong-scaling", num_args(1))]
    pub strong_scaling_root: Option<String>,
}
