//! Code used to fetch and hold execution information
//!
//! This module is currently useless but will be built on when introducing
//! parallelism to the program.

use crate::{constants::CustomFloat, parameters::SimulationParameters};

/// Enum used to represent the execution mode of the simulation.
#[derive(Debug, Default, Clone, Copy)]
pub enum ExecPolicy {
    /// Default value. Sequential execution.
    #[default]
    Sequential,
    /// Parallel execution.
    Parallel,
}

/// Structure holding execution information of a given run.
#[derive(Debug)]
pub struct MCProcessorInfo {
    /// Execution mode.
    pub exec_policy: ExecPolicy,
    /// Number of processors of the machine. Currently useless.
    pub num_processors: usize,
    /// Number of thread(s) used for execution.
    pub num_threads: usize,
}

impl MCProcessorInfo {
    /// Constructor. The structure is initialized using parameters and fetched data.
    pub fn new<T: CustomFloat>(sim_params: &SimulationParameters<T>) -> Self {
        // fetch data & init
        let num_threads: usize = sim_params.n_threads as usize;
        let exec_policy = if num_threads > 1 {
            ExecPolicy::Parallel
        } else {
            // if n_threads == 0, default to sequential execution
            ExecPolicy::Sequential
        };
        Self {
            exec_policy,
            num_processors: 1, // need to fetch this?
            num_threads,
        }
    }
}

impl Default for MCProcessorInfo {
    fn default() -> Self {
        Self {
            exec_policy: Default::default(),
            num_processors: 1,
            num_threads: 1,
        }
    }
}
