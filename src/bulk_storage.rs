use std::rc::Rc;

#[derive(Debug)]
pub struct BulkStorage<T> {
    bulk_storage: Rc<Vec<T>>,
    size: usize,
    capacity: usize,
}