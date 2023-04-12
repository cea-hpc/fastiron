//! Code used to fetch and hold execution information
//!
//! This module is currently useless but will be built on when introducing
//! parallelism to the program.

use crate::{constants::CustomFloat, parameters::SimulationParameters};

#[derive(Debug, Default)]
pub enum ExecPolicy {
    #[default]
    Sequential,
    Parallel,
}

#[derive(Debug)]
pub struct MCProcessorInfo {
    pub exec_policy: ExecPolicy,
    pub num_processors: usize,
    pub num_threads: usize,
}

impl MCProcessorInfo {
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
