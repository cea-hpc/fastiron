use num::Float;

use super::mc_domain::MCDomain;

/// Structure used to divide and manage physical space of the problem.
#[derive(Debug, PartialEq, Default)]
pub struct MCLocation {
    pub domain: Option<usize>, // u32? usize? usize would be good but we need a special default value; usize::MAX?
    pub cell: Option<usize>,
    pub facet: Option<usize>,
}

/// TODO: replace calls to this function; avoid lifetime or copy issues
impl MCLocation {
    pub fn get_domain<T: Float>(&self) -> &MCDomain<T> {
        //&mcco.domain[self.domain.unwrap()]
        todo!()
    }
}
