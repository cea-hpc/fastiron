use crate::{montecarlo::MonteCarlo, parameters::Parameters, constants::CustomFloat};

/// Adjust some data for the coral benchmark if it's running.
pub fn coral_benchmark_correctness<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    let params: &Parameters = &mcco.params;
    if !params.simulation_params.coral_benchmark {
        return;
    }
    todo!()
}
