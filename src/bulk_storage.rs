use std::rc::Rc;

#[derive(Debug)]
pub struct BulkStorage<T> {
    bulk_storage: Rc<Vec<T>>,
    size: usize,
    capacity: usize,
}

impl<T> BulkStorage<T> {
    /// Constructor from another one.
    pub fn new(aa: BulkStorage<T>) -> Self {
        todo!()
    }

    /// Sets capacity of the storage ... Exact behavior TBD
    pub fn set_capacity(&mut self, capacity: usize) {
        todo!()
    }

    pub fn get_block(&mut self, n_items: usize) -> Vec<T> {
        todo!()
    }
}

impl<T> Default for BulkStorage<T> {
    fn default() -> Self {
        todo!()
    }
}

// We want a custom behavior when cloning this structure
impl<T> Clone for BulkStorage<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}
