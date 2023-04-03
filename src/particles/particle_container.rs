use crate::{constants::CustomFloat, data::send_queue::SendQueue};

use super::mc_base_particle::MCBaseParticle;

/// Structure used as a container for all particles.
pub struct ParticleContainer<T: CustomFloat> {
    /// Container for particles that have yet to be processed.
    pub processing_particles: Vec<MCBaseParticle<T>>,
    /// Container for already processed particles.
    pub processed_particles: Vec<MCBaseParticle<T>>,
    /// Container for extra particles. This is used for fission-induced
    /// particles and incoming off-processor particles.
    pub extra_particles: Vec<MCBaseParticle<T>>,
    /// Queue used to save particles and neighbor index for any particles
    /// that hit TransitOffProcessor (See MCSubfacetAdjacencyEvent)
    pub send_queue: SendQueue<T>,
}

impl<T: CustomFloat> ParticleContainer<T> {
    /// Processes the particles stored in the send queue.
    /// - In a shared memory context, this is just a transfer from the send queue
    ///   to the extra storage
    /// - In a message-passing context, this would include sending and receiving
    ///   particles
    pub fn process_sq(&mut self) {
        self.send_queue.data.iter().for_each(|sq_tuple| {
            // Neighbor index would be used here to get the correct sender
            // match sq_tuple.neighbor {...}
            self.extra_particles
                .push(MCBaseParticle::new(&sq_tuple.particle));
        });
        self.send_queue.clear();
        // Here we would add the receiver part
        // while rx.try_recv().is_ok() {...}
    }

    /// Adds back to the processing storage the extra particles.
    pub fn clean_extra_vaults(&mut self) {
        self.processing_particles.append(&mut self.extra_particles);
    }

    /// Checks if there are no more particles to process, i.e:
    /// - extra storage is empty
    /// - processing storage is empty
    /// - send queue is empty
    pub fn test_done_new(&self) -> bool {
        self.extra_particles.is_empty()
            & self.processing_particles.is_empty()
            & self.send_queue.data.is_empty()
    }
}
