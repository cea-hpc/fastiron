//! Code used for inter-processor particle communication
//!
//! This module contains code to handle data that should be transfered between
//! processors in a parallel context.

use crate::{constants::CustomFloat, particles::mc_particle::MCParticle};

/// Structure to record which particles need to be sent to
/// which neighbor process during tracking.
#[derive(Debug, Clone)]
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
/// [MCSubfacetAdjacencyEvent][crate::geometry::facets::MCSubfacetAdjacencyEvent]
/// for more information.
#[derive(Debug, Clone, Default)]
pub struct SendQueue<T: CustomFloat> {
    /// Buffer structure.
    pub data: Vec<SendQueueTuple<T>>,
}

impl<T: CustomFloat> SendQueue<T> {
    /// Reserve capacity for the queue.
    pub fn reserve(&mut self, size: usize) {
        if self.data.capacity() < size {
            self.data.reserve(size - self.data.capacity());
        }
    }

    /// Get the number of items in SendQueue going to a specific neighbor.
    /// See if it's used and how much it's used. Maybe returning directly a
    /// filtered iterator is more useful.
    pub fn neighbor_size(&self, index: usize) -> usize {
        self.data
            .clone()
            .into_iter()
            .filter(|t| t.neighbor == index)
            .count()
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
    use crate::particles::mc_base_particle::MCBaseParticle;

    use super::*;

    #[test]
    fn reserve() {
        let tt: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 0,
            particle: MCParticle::default(),
        };
        let mut queue = SendQueue { data: vec![tt; 10] };

        assert_eq!(queue.data.len(), 10);
        queue.reserve(20);
        assert_eq!(queue.data.capacity(), 20);
    }

    #[test]
    fn neighbor_size() {
        let tt: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 0,
            particle: MCParticle::default(),
        };
        let mut data = vec![tt; 10];
        data[0].neighbor = 0;
        data[1].neighbor = 1;
        data[2].neighbor = 1;
        data[3].neighbor = 4;
        data[4].neighbor = 3;
        data[5].neighbor = 6;
        data[6].neighbor = 1;
        data[7].neighbor = 3;
        data[8].neighbor = 0;
        data[9].neighbor = 5;
        let queue = SendQueue { data };

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
        let tt: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 0,
            particle: MCParticle::default(),
        };
        let ttt: SendQueueTuple<f64> = SendQueueTuple {
            neighbor: 3,
            particle: MCParticle {
                base_particle: MCBaseParticle {
                    identifier: 23,
                    ..Default::default()
                },
                ..Default::default()
            },
        };

        let mut queue = SendQueue { data: vec![tt; 4] };

        let mut pp = MCParticle::default();
        pp.base_particle.identifier = 23;
        queue.push(3, &pp);

        assert_eq!(queue.data.len(), 5);
        assert_eq!(queue.data[queue.data.len() - 1].neighbor, ttt.neighbor);
        assert_eq!(
            queue.data[queue.data.len() - 1]
                .particle
                .base_particle
                .identifier,
            ttt.particle.base_particle.identifier
        );

        queue.clear();

        assert_eq!(queue.data.len(), 0);
    }
}
