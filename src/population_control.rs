use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::montecarlo::MonteCarlo;

/// Routine used to monitor and regulate population level.
pub fn population_control<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>, load_balance: bool) {
    todo!()
}

/// Play russian-roulette with low-weight particles relative
/// to the source particle weight.
pub fn roulette_low_weight_particles<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    todo!()
}
