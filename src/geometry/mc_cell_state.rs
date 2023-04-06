//! Contains code to represent a cell's current state.

use crate::constants::CustomFloat;

/// Structure used to represent a cell's state, i.e.
/// properties relevant to the simulation.
///
/// Note that in advanced Monte-Carlo code, the mesh may be deformed and
/// a cell's volume may change, hence the cache-ing of that value.
#[derive(Debug, Clone, Default)]
pub struct MCCellState<T: CustomFloat> {
    /// Global id of the material the cell is made of.
    pub material: usize,
    /// Cache for the total cross-sections of energy groups.
    pub total: Vec<T>,
    /// Cell volume in cm³.
    pub volume: T,
    /// Density of the material in the cell?
    pub cell_number_density: T,
    /// Cell identifier.
    pub id: usize,
    /// Local tally counting particles spawned in this cell.
    pub source_tally: usize,
}
