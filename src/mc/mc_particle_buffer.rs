use std::{cell::RefCell, rc::Rc};

use num::{Float, FromPrimitive};

use crate::montecarlo::MonteCarlo;

use super::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle};

/// Structure used as a buffer for particles crossing into different domains.
/// "Useless" in single threaded mode, but will be useful if parallelizing
/// on space division. Should probably be deleted asap.
#[derive(Debug)]
pub struct MCParticleBuffer<T: Float + FromPrimitive> {
    /// Reference to the MonteCarlo object for ease of access.
    pub mcco: Rc<RefCell<MonteCarlo<T>>>,
    /// One buffer per domain: buffers.len()==mcco.domain.len().
    /// The indexing is coherent (buffer of domain[N] == buffers[N])
    pub buffers: Vec<Vec<MCParticle<T>>>,
}

impl<T: Float + FromPrimitive> MCParticleBuffer<T> {
    /// Prepare the buffers for use.
    pub fn initialize(&mut self) {
        self.buffers = Vec::with_capacity(self.mcco.borrow().domain.len());
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
    pub fn test_done_new(&self) -> bool {
        if (self
            .mcco
            .borrow()
            .particle_vault_container
            .send_queue
            .size()
            == 0)
            & self.is_empty()
            & (self
                .mcco
                .borrow()
                .particle_vault_container
                .processing_size()
                == 0)
        {
            // with these three conditions, there should be a bit of
            // leeway as to where we can call the function
            return true;
        }
        false
    }

    /// Put the given MCParticle in the corresponding buffer. The buffer
    /// indexing is coherent with the neighbor indexing so that the
    /// function can easily be called using the two elements of a
    /// SendQueueTuple.
    pub fn buffer_particle(&mut self, mcb_particle: MCBaseParticle<T>, buffer_idx: usize) {
        self.buffers[buffer_idx].push(MCParticle::new(&mcb_particle));
    }

    /// Read the buffers and unpack the particles in the given vault.
    /// Since we are not parallelizing over a spatial division, this
    /// function just unpacks everything.
    pub fn read_buffers(&mut self, fill_vault: &mut usize) {
        // If we were parallelizing, we would add a condition for
        // unpacking like (current thread nbr == buffer nbr)
        // instead of just iterating over all buffers.
        self.buffers.iter().for_each(|b| {
            b.iter().for_each(|particle| {
                self.mcco
                    .borrow_mut()
                    .particle_vault_container
                    .add_processing_particle(MCBaseParticle::new(particle), fill_vault)
            })
        });
        // maybe calling clear() directly here is best?
    }

    /// Clear the buffers
    pub fn clear(&mut self) {
        self.buffers.iter_mut().for_each(|b| b.clear());
    }
}
