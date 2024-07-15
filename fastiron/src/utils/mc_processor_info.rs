//! Code used to fetch and hold execution information
//!
//! This module is currently useless but will be built on when introducing
//! parallelism to the program.

use std::fmt::Display;

use crate::{constants::CustomFloat, parameters::SimulationParameters};

/// Enum used to represent the execution mode of the simulation.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ExecPolicy {
    /// Default value. Sequential execution.
    #[default]
    Sequential,
    Rayon,
    Distributed,
    Hybrid,
}

impl Display for ExecPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ExecPolicy::Sequential => write!(f, "Sequential"),
            ExecPolicy::Rayon => write!(f, "Rayon Only"),
            ExecPolicy::Distributed => write!(f, "Distributed"),
            ExecPolicy::Hybrid => write!(f, "Hybrid"),
        }
    }
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
    /// Switch to bind rayon threads to physical cores.
    pub bind_threads: bool,
    /// Size of the chunks used by rayon.
    pub chunk_size: usize,
    /// Number of unit(s) used for (distributed) execution.
    pub n_units: usize,
}

impl MCProcessorInfo {
    /// Constructor. The structure is initialized using parameters and fetched data.
    pub fn new<T: CustomFloat>(sim_params: &SimulationParameters<T>) -> Self {
        let mut res = MCProcessorInfo::default();
        // fetch data & init
        if sim_params.n_units != 1 {
            res.n_units = sim_params.n_units as usize;
            if sim_params.n_rayon_threads != 1 {
                res.n_rayon_threads = sim_params.n_rayon_threads as usize;
                res.exec_policy = ExecPolicy::Hybrid;
            } else {
                res.exec_policy = ExecPolicy::Distributed;
            }
        } else if sim_params.n_rayon_threads != 1 {
            res.n_rayon_threads = sim_params.n_rayon_threads as usize;
            res.bind_threads = sim_params.bind_threads;
            res.exec_policy = ExecPolicy::Rayon;
        };
        res.chunk_size = sim_params.chunk_size as usize;

        assert_ne!(res.n_units, 0);
        // we allow 0 for rayon control
        // A value of 0 means we use the default number of threads
        // chosen by rayon in an implicit init
        // assert_ne!(res.n_rayon_threads, 0);
        res
    }
}

impl Default for MCProcessorInfo {
    fn default() -> Self {
        Self {
            exec_policy: Default::default(),
            n_processors: 1,
            n_rayon_threads: 1,
            bind_threads: false,
            chunk_size: 0,
            n_units: 1,
        }
    }
}

impl Display for MCProcessorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  Policy: {}", self.exec_policy).unwrap();
        writeln!(f, "  Number of processors: {}", self.n_processors).unwrap();
        match self.exec_policy {
            ExecPolicy::Sequential => Ok(()),
            ExecPolicy::Rayon => writeln!(f, "  Number of rayon threads: {}", self.n_rayon_threads),
            ExecPolicy::Distributed => writeln!(f, "  Number of units: {}", self.n_units),
            ExecPolicy::Hybrid => {
                writeln!(f, "  Number of rayon threads: {}", self.n_rayon_threads).unwrap();
                writeln!(f, "  Number of units: {}", self.n_units)
            }
        }
    }
}
