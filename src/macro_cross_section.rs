use num::Float;

use crate::montecarlo::MonteCarlo;

/// Computes the number-density-weighted macroscopic cross section
/// of a cell. A reaction index of ? means total cross section.
pub fn macroscopic_cross_section<T: Float>(
    mcco: &MonteCarlo<T>,
    reaction_idx: usize,
    domain_idx: usize,
    cell_idx: usize,
    isotope_idx: usize,
    energy_group: usize,
) -> T {
    todo!()
}

/// Computes the number-density-weighted macroscopic cross section
/// of the collection of isotopes in a cell.
pub fn weighted_macroscopic_cross_section<T: Float>(
    mcco: &MonteCarlo<T>,
    task_idx: usize,
    domain_idx: usize,
    cell_idx: usize,
    energy_group: usize,
) -> T {
    todo!()
}
