use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::{montecarlo::MonteCarlo, send_queue::SendQueue};

use super::mc_particle::MCParticle;

#[derive(Debug)]
pub struct MCPTestDone {}

#[derive(Debug)]
pub struct MCParticleBuffer<T: Float> {
    /// Reference to the MonteCarlo object for ease of access.
    pub mcco: Rc<RefCell<MonteCarlo<T>>>,
    /// One buffer per domain: buffers.len()==mcco.domain.len()
    pub buffers: Vec<Vec<MCParticle<T>>>,
}

impl<T: Float> MCParticleBuffer<T> {
    pub fn initialize(&mut self) {
        todo!()
    }

    pub fn test_done_new(&self) -> bool {
        todo!()
    }

    pub fn get_domain_buffer(&self) -> &[MCParticle<T>] {
        todo!()
    }

    pub fn update_buffers(&mut self, send_q: &SendQueue) {}

    pub fn clear(&mut self) {}
}
