//! Code originally used to allocate domains to MPI ranks
//!
//! **May be removed or reworked to make use of the neighboring system for
//! threading**.

/// Object used to record which domains belongs to which rank.
///
/// # How it works
///
/// Indexing between `rank` and `index` fields is coherent meaning that all
/// the domains indexing is held by these two vectors: the global identifier of
/// a given domain can be computed using the rank it belongs to and its local index.
/// Global identifiers of the assigned domains are computed this way.
#[derive(Debug)]
pub struct DecompositionObject {
    /// Global identifiers of the assigned domains.
    pub assigned_gids: Vec<usize>,
    /// Ranks of all the domains
    pub rank: Vec<usize>,
    /// Local indexes of the all domains.
    pub index: Vec<usize>,
}

impl DecompositionObject {
    /// Constructor
    pub fn new(my_rank: usize, n_ranks: usize, dom_per_rank: usize) -> Self {
        let n_domains = n_ranks * dom_per_rank;
        let mut rank: Vec<usize> = Vec::with_capacity(n_domains);
        let mut index: Vec<usize> = Vec::with_capacity(n_domains);
        let mut assigned_gids: Vec<usize> = Vec::with_capacity(dom_per_rank);

        (0..n_domains).for_each(|domain_idx| {
            rank.push(domain_idx / dom_per_rank);
            index.push(domain_idx % dom_per_rank);
        });

        (0..dom_per_rank).for_each(|ii| {
            let idx = dom_per_rank * my_rank + ii;
            assigned_gids.push(dom_per_rank * rank[idx] + index[idx]);
        });

        Self {
            assigned_gids,
            rank,
            index,
        }
    }
}
