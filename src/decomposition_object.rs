#[derive(Debug)]
pub struct DecompositionObject {
    pub assigned_gids: Vec<usize>,
    pub rank: Vec<usize>,
    pub index: Vec<usize>,
}

impl DecompositionObject {
    pub fn new(my_rank: usize, n_ranks: usize, dom_per_rank: usize, mode: bool) -> Self {
        todo!()
    }
}
