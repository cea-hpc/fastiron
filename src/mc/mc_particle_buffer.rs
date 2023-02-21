use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::montecarlo::MonteCarlo;

use super::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle};

#[derive(Debug)]
pub struct MCPTestDone {}

#[derive(Debug)]
pub struct MCParticleBuffer<T: Float> {
    /// Reference to the MonteCarlo object for ease of access.
    pub mcco: Rc<RefCell<MonteCarlo<T>>>,
    /// One buffer per domain: buffers.len()==mcco.domain.len().
    /// The indexing is coherent (buffer of domain[N] == buffers[N])
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

    /// Put the given MCParticle in the corresponding buffer. The buffer
    /// indexing is coherent with the neighbor indexing.
    pub fn buffer_particle(&mut self, mcb_particle: MCBaseParticle<T>, buffer_idx: usize) {
        todo!()
    }

    /// Read the buffers and unpack the particles in the given vault.
    pub fn read_buffers(&mut self, fill_vault: &mut usize) {
        todo!()
    }

    pub fn clear(&mut self) {}
}
