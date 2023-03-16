use crate::constants::CustomFloat;

/// Structure used to keep track of the progress of the
/// Monte Carlo algorithm.
#[derive(Debug, Default)]
pub struct MCTimeInfo<T: CustomFloat> {
    // generic T or forced f64 for time management?
    pub cycle: u32,
    pub initial_time: T,
    pub final_time: T,
    pub time: T,
    pub time_step: T,
}
