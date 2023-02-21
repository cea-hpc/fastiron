use std::{cell::RefCell, rc::Rc};

use num::Float;

use crate::montecarlo::MonteCarlo;

use super::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle};

#[derive(Debug)]
pub struct MCPTestDone {}

/// Structure used as a buffer for particles crossing into different domains.
/// "Useless" in single threaded mode, but will be useful if parallelizing
/// on space division.
#[derive(Debug)]
pub struct MCParticleBuffer<T: Float> {
    /// Reference to the MonteCarlo object for ease of access.
    pub mcco: Rc<RefCell<MonteCarlo<T>>>,
    /// One buffer per domain: buffers.len()==mcco.domain.len().
    /// The indexing is coherent (buffer of domain[N] == buffers[N])
    pub buffers: Vec<Vec<MCParticle<T>>>,
}

impl<T: Float> MCParticleBuffer<T> {
    /// Prepare the buffers for use.
    pub fn initialize(&mut self) {
        todo!()
    }

    /// Check if there are no more particle transfer?
    pub fn test_done_new(&self) -> bool {
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

    /// Clear the buffers
    pub fn clear(&mut self) {}
}
