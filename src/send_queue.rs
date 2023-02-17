use std::collections::VecDeque;

/// Structure to record which particles need to be sent to
/// which neighbor process during tracking.
#[derive(Debug)]
pub struct SendQueueTuple {
    pub neighbor: u32,
    pub particle_index: usize,
}

/// Structure used to store particle index and neighbor index
/// for particles that hit TransitOffProcessor (See MCSubfacetAdjacencyEvent).
#[derive(Debug)]
pub struct SendQueue {
    data: VecDeque<SendQueueTuple>,
}

impl SendQueue {
    /// Get the total size of the SendQueue.
    fn size(&self) -> usize {
        todo!()
    }

    /// Reserve capacity ... Exact behavior TBD
    fn reserve(&self, size: usize) {
        todo!()
    }

    /// Get the number of items in SendQueue going to a specific neighbor.
    fn neighbor_size(&self, index: usize) -> u64 {
        todo!()
    }

    /// Get a [SendQueueTuple] from the SendQueue. `index`is the index
    /// of the desination neighbor i.e. the current process id?
    fn get_tuple(&self, index: usize) -> SendQueueTuple {
        todo!()
    }

    /// Add items to the SendQueue ... Exact behavior TBD
    fn push(&mut self, neighbor: u32, vault_index: usize) {
        todo!()
    }

    /// Clear the queue.
    fn clear(&mut self) {

    }
}
