//! Code used to keep track of the progress of the simulation

use crate::constants::CustomFloat;

/// Structure used to keep track of the progress of the
/// Monte Carlo algorithm.
#[derive(Debug, Default)]
pub struct MCTimeInfo<T: CustomFloat> {
    /// Current cycle number.
    pub cycle: u32,
    /// Time step of the simulation in seconds.
    pub time_step: T,
}
