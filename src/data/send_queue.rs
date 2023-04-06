use crate::{constants::CustomFloat, particles::mc_particle::MCParticle};

/// Structure to record which particles need to be sent to
/// which neighbor process during tracking.
#[derive(Debug, Clone, PartialEq)]
pub struct SendQueueTuple<T: CustomFloat> {
    pub neighbor: usize,
    pub particle: MCParticle<T>,
}

/// Structure used to store particle index and neighbor index
/// for particles that hit TransitOffProcessor (See MCSubfacetAdjacencyEvent).
#[derive(Debug, Clone)]
pub struct SendQueue<T: CustomFloat> {
    pub data: Vec<SendQueueTuple<T>>,
}

impl<T: CustomFloat> SendQueue<T> {
    /// Get the total size of the SendQueue.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Reserve capacity for the queue
    pub fn reserve(&mut self, size: usize) {
        if self.data.capacity() < size {
            self.data.reserve(size - self.data.capacity());
        }
    }

    /// Get the number of items in SendQueue going to a specific neighbor.
    /// See if it's used and how much it's used. Maybe returning directly a
    /// filtered iterator is more useful.
    pub fn neighbor_size(&self, index: usize) -> u64 {
        self.data
            .clone()
            .into_iter()
            .filter(|t| t.neighbor == index)
            .count() as u64
    }

    /// Add items to the SendQueue.
    pub fn push(&mut self, neighbor: usize, pp: &MCParticle<T>) {
        self.data.push(SendQueueTuple {
            neighbor,
            particle: pp.clone(),
        });
    }

    /// Clear the queue.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
