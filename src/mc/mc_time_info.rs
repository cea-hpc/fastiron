/// Structure used to keep track of the progress of the
/// Monte Carlo algorithm.
#[derive(Debug)]
pub struct MCTimeInfo {
    pub cycle: u32,
    pub initial_time: f64,
    pub final_time: f64,
    pub time: f64,
    pub time_step: f64,
}
