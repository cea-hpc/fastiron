/// Structure to record which particles need to be sent to
/// which neighbor process during tracking.
#[derive(Debug, Clone, PartialEq)]
pub struct SendQueueTuple {
    pub neighbor: usize,
    pub particle_index: usize,
}

/// Structure used to store particle index and neighbor index
/// for particles that hit TransitOffProcessor (See MCSubfacetAdjacencyEvent).
/// It is called a Queue but no properties of a queue are used in the 
/// original code; Replace it?
#[derive(Debug, Clone)]
pub struct SendQueue {
    pub data: Vec<SendQueueTuple>,
}

impl SendQueue {
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
    pub fn neighbor_size(&self, index: usize) -> u64 {
        self.data.clone().into_iter().filter(|t| t.neighbor == index).count() as u64
    }   

    /// Get a [SendQueueTuple] from the SendQueue. `index`is the index
    /// of the desination neighbor i.e. the current process id?
    pub fn get_tuple(&self, index: usize) -> Option<SendQueueTuple> {
        self.data.get(index).cloned() // return a clone or a ref or pop it ?
    }

    /// Add items to the SendQueue
    pub fn push(&mut self, neighbor: usize, vault_index: usize) {
        self.data.push(SendQueueTuple { neighbor, particle_index: vault_index });
    }

    /// Clear the queue.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
