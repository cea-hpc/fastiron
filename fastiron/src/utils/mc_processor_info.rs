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
    Rayon,
    Distributed,
    Hybrid,
}

/// Structure holding execution information of a given run.
#[derive(Debug)]
pub struct MCProcessorInfo {
    /// Execution mode.
    pub exec_policy: ExecPolicy,
    /// Number of processors of the machine. Currently useless.
    pub n_processors: usize,
    /// Number of thread(s) used for execution.
    pub n_rayon_threads: usize,
    /// Number of unit(s) used for (distributed) execution.
    pub n_units: usize,
}

impl MCProcessorInfo {
    /// Constructor. The structure is initialized using parameters and fetched data.
    pub fn new<T: CustomFloat>(sim_params: &SimulationParameters<T>) -> Self {
        // fetch data & init
        todo!()
    }
}

impl Default for MCProcessorInfo {
    fn default() -> Self {
        Self {
            exec_policy: Default::default(),
            n_processors: 1,
            n_rayon_threads: 1,
            n_units: 1,
        }
    }
}
