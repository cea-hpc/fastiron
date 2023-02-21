/// Structure used to divide and manage physical space of the problem.
#[derive(Debug, PartialEq)]
pub struct MCLocation {
    pub domain: u32, // u32? usize? usize would be good but we need a special default value; usize::MAX?
    pub cell: u32,
    pub facet: u32,
}

impl Default for MCLocation {
    fn default() -> Self {
        todo!()
    }
}
