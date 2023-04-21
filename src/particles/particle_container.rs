//! Data structure used to hold particles
//!
//! This module contains code used for the main structure holding particles.

use crate::{constants::CustomFloat, data::send_queue::SendQueue};

use super::mc_particle::MCParticle;

#[derive(Debug, Clone)]
/// Structure used as a container for all particles.
///
/// The [Clone] implementation should not be used except at the beginning of the program.
pub struct ParticleContainer<T: CustomFloat> {
    /// Container for particles that have yet to be processed.
    pub processing_particles: Vec<MCParticle<T>>,
    /// Container for already processed particles.
    pub processed_particles: Vec<MCParticle<T>>,
    /// Container for extra particles. This is used for fission-induced
    /// particles and incoming off-processor particles.
    pub extra_particles: Vec<MCParticle<T>>,
    /// Queue used to save particles and neighbor index for any particles
    /// moving from a domain managed by a different processor than the current
    /// one.
    pub send_queue: SendQueue<T>,
}

impl<T: CustomFloat> ParticleContainer<T> {
    /// Constructor. The appropriate capacity is computed beforehand.
    pub fn new(regular_capacity: usize, extra_capacity: usize) -> Self {
        Self {
            processing_particles: Vec::with_capacity(regular_capacity),
            processed_particles: Vec::with_capacity(regular_capacity),
            extra_particles: Vec::with_capacity(extra_capacity),
            send_queue: Default::default(),
        }
    }

    /// Swap the processing and processed particle lists. This function is used in-between
    /// iterations.
    pub fn swap_processing_processed(&mut self) {
        core::mem::swap(
            &mut self.processing_particles,
            &mut self.processed_particles,
        );
    }

    /// Processes the particles stored in the send queue.
    /// - In a shared memory context, this is just a transfer from the send queue
    ///   to the extra storage
    /// - In a message-passing context, this would include sending and receiving
    ///   particles
    pub fn process_sq(&mut self) {
        self.send_queue.data.iter().for_each(|sq_tuple| {
            // Neighbor index would be used here to get the correct sender
            // match sq_tuple.neighbor {...}
            self.extra_particles.push(sq_tuple.particle.clone());
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
