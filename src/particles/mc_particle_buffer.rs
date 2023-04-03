use std::fmt::Debug;

use crate::{constants::CustomFloat, montecarlo::MonteCarlo};

use super::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle};

/// Structure used as a buffer for particles crossing into different domains.
/// "Useless" in single threaded mode, but will be useful if parallelizing
/// on space division. Should probably be deleted asap.
#[derive(Debug, Default)]
pub struct MCParticleBuffer<T: CustomFloat> {
    /// One buffer per domain: buffers.len()==mcco.domain.len().
    /// The indexing is coherent (buffer of domain[N] == buffers[N])
    pub buffers: Vec<Vec<MCParticle<T>>>,
}

impl<T: CustomFloat> MCParticleBuffer<T> {
    /// Prepare the buffers for use.
    pub fn initialize(&mut self, len: usize) {
        self.buffers = Vec::with_capacity(len);
    }

    /// Returns true if all buffers are empty
    pub fn is_empty(&self) -> bool {
        for bb in self.buffers.iter() {
            if !bb.is_empty() {
                return false;
            }
        }
        true
    }

    /// Check if there are no more particle transfer. The exact conditions
    /// to look for might change.
    pub fn test_done_new(&self, mcco: &MonteCarlo<T>) -> bool {
        let buffer_empty = self.is_empty();
        let sendq_empty = mcco.particle_vault_container.send_queue.size() == 0;
        let processing_empty = mcco.particle_vault_container.particles_processing_size() == 0;

        buffer_empty & sendq_empty & processing_empty
    }

    /// Put the given MCParticle in the corresponding buffer. The buffer
    /// indexing is coherent with the neighbor indexing so that the
    /// function can easily be called using the two elements of a
    /// SendQueueTuple.
    pub fn buffer_particle(&mut self, base_particle: MCBaseParticle<T>, buffer_idx: usize) {
        self.buffers[buffer_idx].push(MCParticle::new(&base_particle));
    }

    /// Clear the buffers
    pub fn clear(&mut self) {
        self.buffers.iter_mut().for_each(|b| b.clear());
    }
}
