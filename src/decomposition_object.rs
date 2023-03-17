/// Object used to allocate domains to rank. Somewhat useless currently.
#[derive(Debug)]
pub struct DecompositionObject {
    pub assigned_gids: Vec<usize>,
    pub rank: Vec<usize>,
    pub index: Vec<usize>,
}

impl DecompositionObject {
    /// Constructor
    pub fn new(my_rank: usize, n_ranks: usize, dom_per_rank: usize) -> Self {
        let n_domains = n_ranks * dom_per_rank;
        let mut rank: Vec<usize> = Vec::with_capacity(n_domains);
        let mut index: Vec<usize> = Vec::with_capacity(n_domains);
        let mut assigned_gids: Vec<usize> = Vec::with_capacity(dom_per_rank);

        (0..n_domains).into_iter().for_each(|domain_idx| {
            rank.push(domain_idx / dom_per_rank);
            index.push(domain_idx % dom_per_rank);
        });

        (0..dom_per_rank).into_iter().for_each(|ii| {
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
