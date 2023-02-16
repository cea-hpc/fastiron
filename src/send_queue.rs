use std::collections::VecDeque;

/// Structure to record which particles need to be sent to
/// which neighbor process during tracking.
#[derive(Debug)]
pub struct SendQueueTuple {
    pub neighbor: u32,
    pub particle_index: u32, // usize?
}

/// Structre used to store particle index and neighbor index
/// for particles that hit TransitOffProcessor (See MCSubfacetAdjacencyEvent).
#[derive(Debug)]
pub struct SendQueue {
    data: VecDeque<SendQueueTuple>,
}
