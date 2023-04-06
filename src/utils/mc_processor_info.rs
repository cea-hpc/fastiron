//! Code used to fetch and hold execution information
//!
//! This module is currently useless but will be built on when introducing
//! parallelism to the program.

#[derive(Debug, Default)]
pub enum ExecPolicy {
    #[default]
    Sequential,
}

#[derive(Debug)]
pub struct MCProcessorInfo {
    pub exec_policy: ExecPolicy,
    pub num_processors: usize,
}

impl MCProcessorInfo {
    pub fn new() -> Self {
        // fetch data & init
        Self::default()
    }
}

impl Default for MCProcessorInfo {
    fn default() -> Self {
        Self {
            exec_policy: Default::default(),
            num_processors: 1,
        }
    }
}
