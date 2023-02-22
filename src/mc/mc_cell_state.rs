use num::Float;

/// Structure used to represent a cell's state, i.e.
/// properties relevant to the simulation.
#[derive(Debug)]
pub struct MCCellState<T: Float> {
    /// Global id of the material (should be usize?)
    pub material: u32,
    /// Energy groups
    pub total: Vec<T>,
    /// Cell volume
    pub volume: T,
    /// ?
    pub cell_number_density: T,
    /// Cell identifier
    pub id: usize,
    /// ?
    pub source_tally: usize,
}

impl<T: Float> Default for MCCellState<T> {
    fn default() -> Self {
        todo!()
    }
}
