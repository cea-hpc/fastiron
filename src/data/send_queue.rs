//! Code used for inter-processor particle communication
//!
//! This module contains code to handle data that should be transfered between
//! processors in a parallel context.

use crate::{constants::CustomFloat, particles::mc_particle::MCParticle};

/// Structure to record which particles need to be sent to
/// which neighbor process during tracking.
#[derive(Debug, Clone, PartialEq)]
pub struct SendQueueTuple<T: CustomFloat> {
    pub neighbor: usize,
    pub particle: MCParticle<T>,
}

/// Structure used to store particles and neighbor index
/// for particles that hit TransitOffProcessor.
///
/// Particles may cross to cells that are managed by a domain managed by a
/// different processor. In this case, they get are buffered for transfer by
/// this structure. See
/// [MCSubfacetAdjacencyEvent][crate::geometry::mc_facet_adjacency::MCSubfacetAdjacencyEvent]
/// for more information.
#[derive(Debug, Clone, Default)]
pub struct SendQueue<T: CustomFloat> {
    /// Buffer structure.
    pub data: Vec<SendQueueTuple<T>>,
}

impl<T: CustomFloat> SendQueue<T> {
    /// Get the total size of the SendQueue.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Reserve capacity for the queue.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserve() {
        let tt: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 0,
            particle: MCParticle::default(),
        };
        let mut queue = SendQueue { data: vec![tt; 10] };

        assert_eq!(queue.size(), 10);
        queue.reserve(20);
        assert_eq!(queue.data.capacity(), 20);
    }

    #[test]
    fn neighbor_size() {
        let t0: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 0,
            particle: MCParticle::default(),
        };
        let t1: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 1,
            particle: MCParticle::default(),
        };
        let t2: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 1,
            particle: MCParticle::default(),
        };
        let t3: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 4,
            particle: MCParticle::default(),
        };
        let t4: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 3,
            particle: MCParticle::default(),
        };
        let t5: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 6,
            particle: MCParticle::default(),
        };
        let t6: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 1,
            particle: MCParticle::default(),
        };
        let t7: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 3,
            particle: MCParticle::default(),
        };
        let t8: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 0,
            particle: MCParticle::default(),
        };
        let t9: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 5,
            particle: MCParticle::default(),
        };

        let queue = SendQueue {
            data: vec![t0, t1, t2, t3, t4, t5, t6, t7, t8, t9],
        };

        assert_eq!(queue.neighbor_size(0), 2);
        assert_eq!(queue.neighbor_size(1), 3);
        assert_eq!(queue.neighbor_size(2), 0);
        assert_eq!(queue.neighbor_size(3), 2);
        assert_eq!(queue.neighbor_size(4), 1);
        assert_eq!(queue.neighbor_size(5), 1);
        assert_eq!(queue.neighbor_size(6), 1);
        assert_eq!(queue.neighbor_size(7), 0);
    }

    #[test]
    fn push_get_clear() {
        let t0: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 0,
            particle: MCParticle::default(),
        };
        let t1: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 1,
            particle: MCParticle::default(),
        };
        let t2: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 1,
            particle: MCParticle::default(),
        };
        let t3: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 4,
            particle: MCParticle::default(),
        };
        let t4: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 3,
            particle: MCParticle::default(),
        };

        let mut queue = SendQueue {
            data: vec![t0, t1, t2, t3],
        };
        queue.push(3, &MCParticle::default());

        assert_eq!(queue.size(), 5);
        assert_eq!(queue.data[queue.size() - 1], t4);

        queue.clear();

        assert_eq!(queue.size(), 0);
    }
}
