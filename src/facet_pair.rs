use crate::mc::mc_location::MCLocation;

/// Structure used to pair facet that joins two domains.
/// Replace fields with location directly?
#[derive(Debug)]
pub struct FacetPair {
    pub domain_gid1: usize,
    pub domain_idx1: Option<usize>,
    pub facet_idx1: Option<usize>,
    pub cell_idx1: Option<usize>,
    pub domain_gid2: usize,
    pub domain_idx2: Option<usize>,
    pub facet_idx2: Option<usize>,
    pub cell_idx2: Option<usize>,
}

impl FacetPair {
    /// Constructor.
    pub fn new(
        domain_gid1: usize,
        loc1: &MCLocation,
        domain_gid2: usize,
        loc2: &MCLocation,
    ) -> Self {
        Self {
            domain_gid1,
            domain_idx1: loc1.domain,
            facet_idx1: loc1.facet,
            cell_idx1: loc1.cell,
            domain_gid2,
            domain_idx2: loc2.domain,
            facet_idx2: loc2.facet,
            cell_idx2: loc2.cell,
        }
    }
}
