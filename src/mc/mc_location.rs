use num::Float;

use crate::montecarlo::MonteCarlo;

use super::mc_domain::MCDomain;

/// Structure used to divide and manage physical space of the problem.
#[derive(Debug, PartialEq)]
pub struct MCLocation {
    pub domain: Option<usize>, // u32? usize? usize would be good but we need a special default value; usize::MAX?
    pub cell: Option<usize>,
    pub facet: Option<usize>,
}

impl Default for MCLocation {
    fn default() -> Self {
        todo!()
    }
}
