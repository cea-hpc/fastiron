use crate::constants::CustomFloat;

/// Structure used to represent a cell's state, i.e.
/// properties relevant to the simulation.
#[derive(Debug, Clone, Default)]
pub struct MCCellState<T: CustomFloat> {
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
