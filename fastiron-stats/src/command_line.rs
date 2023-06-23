//! Module used for I/O handling
//!
//! This module holds all the code defining the CLI, i.e. the entire I/O system of
//! the program.

use clap::{Args, Parser};

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
        requires_all(vec!["t_init", "t_iter", "t_factor"])
    )]
    pub weak_scaling_root: Option<String>,

    /// root path of timers file for strong scaling data
    #[arg(
        short = 'S',
        long = "strong-scaling",
        num_args(1),
        requires_all(vec!["t_init", "t_iter", "t_factor"])
    )]
    pub strong_scaling_root: Option<String>,

    #[command(flatten)]
    pub scaling_params: ScalingParams,

    /// if present, plot the results of all computed metrics
    #[arg(short = 'p', long = "plot", num_args(0))]
    pub plot: bool,
}

/// Structure used to hold all scaling parameters.
#[derive(Args, Debug)]
pub struct ScalingParams {
    /// starting number of threads for scaling data
    #[arg(short = 't', long = "thread_init_n", num_args(1))]
    pub t_init: Option<usize>,

    /// starting number of threads for scaling data
    #[arg(short = 'i', long = "thread_iter_n", num_args(1))]
    pub t_iter: Option<usize>,

    /// starting number of threads for scaling data
    #[arg(short = 'f', long = "thread_step_factor", num_args(1))]
    pub t_factor: Option<usize>,
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli_parsing() {
        use super::*;
        let cmd_line = "./fastiron-stats -E file_path1 -T oldfile newfile --strong-scaling file_path2/rootname_t -t 1 -i 9 -f 2";
        let cli = Cli::parse_from(cmd_line.split(' '));
        assert!(cli.weak_scaling_root.is_none());
        assert_eq!(cli.correlation.unwrap(), "file_path1");
        assert_eq!(
            cli.comparison.unwrap(),
            vec!["oldfile".to_string(), "newfile".to_string()]
        );
        assert_eq!(cli.strong_scaling_root.unwrap(), "file_path2/rootname_t");
        assert_eq!(cli.scaling_params.t_init.unwrap(), 1);
        assert_eq!(cli.scaling_params.t_iter.unwrap(), 9);
        assert_eq!(cli.scaling_params.t_factor.unwrap(), 2);
    }

    #[test]
    fn missing_scaling_param() {
        use super::*;
        let cmd_line = "./fastiron-stats --strong-scaling file_path2/rootname_t -t 1 -f 2";
        let cli = Cli::try_parse_from(cmd_line.split(' '));
        assert!(cli.is_err());
        assert_eq!(
            cli.unwrap_err().kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );
    }
}
