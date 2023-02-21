use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::{montecarlo::MonteCarlo, parameters::Parameters};

/// Adjust some data for the coral benchmark if it's running.
pub fn coral_benchmark_correctness<T: Float>(
    mcco: Rc<RefCell<MonteCarlo<T>>>,
    params: &Parameters,
) {
    todo!()
}
