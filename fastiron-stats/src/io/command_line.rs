use clap::Parser;

/// fastiron-stats, a profiling companion for Fastiron
#[derive(Debug, Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    /// name of the two timers report files to compare - old report first, new report second
    #[arg(short = 'T', long = "timers-comparison", num_args(2))]
    pub comparison: Option<Vec<String>>,

    /// name of the tallies file to analyze
    #[arg(short = 'E', long = "event-correlation", num_args(1))]
    pub correlation: Option<String>,

    /// root path of timers file for weak scaling data
    #[arg(
        short = 'W',
        long = "weak-scaling",
        num_args(1),
        requires("n-start-thread"),
        requires("n-iter-thread"),
        requires("factor-thread")
    )]
    pub weak_scaling_root: Option<String>,

    /// root path of timers file for strong scaling data
    #[arg(
        short = 'S',
        long = "strong-scaling",
        num_args(1),
        requires("n-start-thread"),
        requires("n-iter-thread"),
        requires("factor-thread")
    )]
    pub strong_scaling_root: Option<String>,

    /// starting number of threads for scaling data
    #[arg(short = 't', long = "n-start-thread", num_args(1))]
    pub t_init: Option<usize>,

    /// starting number of threads for scaling data
    #[arg(short = 'i', long = "n-iter-thread", num_args(1))]
    pub t_iter: Option<usize>,

    /// starting number of threads for scaling data
    #[arg(short = 'f', long = "factor-thread", num_args(1))]
    pub t_factor: Option<usize>,
}
