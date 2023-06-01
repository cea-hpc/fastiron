//! Rayon thread pools with threads bound to single numa nodes.
mod bindable_thread_pool;
pub use crate::bindable_thread_pool::{Policy, ThreadPoolBuilder};
