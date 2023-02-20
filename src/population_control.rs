use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::montecarlo::MonteCarlo;

pub fn population_control<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>, load_balance: bool) {
    todo!()
}

pub fn roulette_low_weight_particles<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>) {
    todo!()
}
