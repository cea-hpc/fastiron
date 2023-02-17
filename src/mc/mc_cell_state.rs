/// Structure used to represent a cell's state, i.e.
/// properties relevant to the simulation.
#[derive(Debug)]
pub struct MCCellState {
    /// Global id of the material (should be usize?)
    pub material: u32,
    /// Energy groups
    pub total: Vec<f64>,
    /// Cell volume
    pub volume: f64,
    /// ?
    pub cell_number_density: f64,
    /// Cell identifier
    pub id: usize,
    /// ?
    pub source_tally: usize,
}
