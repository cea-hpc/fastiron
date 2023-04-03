use crate::{constants::CustomFloat, data::send_queue::SendQueue};

use super::mc_particle::MCParticle;

/// Structure used as a container for all particles.
pub struct ParticleContainer<T: CustomFloat> {
    /// Container for particles that have yet to be processed.
    pub processing_particles: Vec<MCParticle<T>>,
    /// Container for already processed particles.
    pub processed_particles: Vec<MCParticle<T>>,
    /// Container for extra particles. This is used for fission-induced
    /// particles and incoming off-processor particles.
    pub extra_particles: Vec<MCParticle<T>>,
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
    pub fn process_sq(&mut self) {}

    /// Adds back to the processing storage the extra particles.
    pub fn clean_extra_vaults(&mut self) {}

    /// Checks if there are no more particles to process.
    pub fn test_done_new(&self) {}
}
