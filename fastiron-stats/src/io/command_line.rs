use clap::Parser;

/// fastiron-stats, a profiling companion for Fastiron
#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    /// name of the two timers report files to compare - old report first, new report second
    #[arg(short = 't', long = "timers-comparison", num_args(2))]
    pub comparison: Option<Vec<String>>,

    /// name of the tallies file to analyze
    #[arg(short = 'e', long = "event-correlation", num_args(1))]
    pub correlation: Option<String>,

    /// root path of timers file for weak scaling data
    #[arg(short = 'W', long = "weak-scaling", num_args(1))]
    pub weak_scaling_root: Option<String>,

    /// root path of timers file for strong scaling data
    #[arg(short = 'S', long = "strong-scaling", num_args(1))]
    pub strong_scaling_root: Option<String>,
}
