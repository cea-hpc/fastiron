use crate::mc::mc_location::MCLocation;

#[derive(Debug)]
pub struct FacetPair {
    pub domain_gid1: usize,
    pub domain_idx1: usize,
    pub facet_idx1: usize,
    pub cell_idx1: usize,
    pub domain_gid2: usize,
    pub domain_idx2: usize,
    pub facet_idx2: usize,
    pub cell_idx2: usize,
}

impl FacetPair {
    pub fn new(
        domain_gid1: usize,
        loc1: &MCLocation,
        domain_gid2: usize,
        loc2: &MCLocation,
    ) -> Self {
        todo!()
    }
}
