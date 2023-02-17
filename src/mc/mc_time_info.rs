use num::Float;

/// Structure used to keep track of the progress of the
/// Monte Carlo algorithm.
#[derive(Debug)]
pub struct MCTimeInfo<T: Float> {
    pub cycle: u32,
    pub initial_time: T,
    pub final_time: T,
    pub time: T,
    pub time_step: T,
}

impl<T: Float> Default for MCTimeInfo<T> {
    fn default() -> Self {
        todo!()
    }
}
