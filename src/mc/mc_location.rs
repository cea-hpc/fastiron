/// Structure used to divide and manage physical space of the problem.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MCLocation {
    pub domain: Option<usize>,
    pub cell: Option<usize>,
    pub facet: Option<usize>,
}
