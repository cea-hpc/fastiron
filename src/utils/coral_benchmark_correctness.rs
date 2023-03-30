use crate::{
    constants::CustomFloat,
    montecarlo::MonteCarlo,
    parameters::{BenchType, Parameters},
};

/// Adjust some data for the coral benchmark if it's running.
pub fn coral_benchmark_correctness<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    let params: &Parameters<T> = &mcco.params;
    if params.simulation_params.coral_benchmark == BenchType::Standard {
        return;
    }
    todo!()
}
