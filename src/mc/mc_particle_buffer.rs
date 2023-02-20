use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::montecarlo::MonteCarlo;

#[derive(Debug)]
pub struct MCParticleBuffer<T: Float> {
    pub mcco: Rc<RefCell<MonteCarlo<T>>>,
}

impl<T: Float> MCParticleBuffer<T> {
    pub fn initialize(&mut self) {
        todo!()
    }

    pub fn test_done_new(&self) -> bool {
        todo!()
    }
}
