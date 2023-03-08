use std::{cell::RefCell, rc::Rc};

use num::{Float, FromPrimitive};

use crate::{montecarlo::MonteCarlo, parameters::Parameters};

/// Adjust some data for the coral benchmark if it's running.
pub fn coral_benchmark_correctness<T: Float + FromPrimitive>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    let params: &Parameters = &mcco.borrow().params;
    todo!()
}
