use num::{zero, Float};

/// Structure used to represent a cell's state, i.e.
/// properties relevant to the simulation.
#[derive(Debug, Clone)]
pub struct MCCellState<T: Float> {
    /// Global id of the material
    pub material: usize,
    /// Energy groups
    pub total: Vec<T>,
    /// Cell volume
    pub volume: T,
    /// density of particles in cell
    pub cell_number_density: T,
    /// Cell identifier
    pub id: usize,
    /// Tally counting for the cell
    pub source_tally: usize,
}

impl<T: Float> Default for MCCellState<T> {
    fn default() -> Self {
        Self {
            material: 0,
            total: Vec::new(),
            volume: zero(),
            cell_number_density: zero(),
            id: 0,
            source_tally: 0,
        }
    }
}
