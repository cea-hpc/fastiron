use num::Float;

use crate::montecarlo::MonteCarlo;

pub fn macroscopic_cross_section<T: Float>(mcco: &MonteCarlo<T>, reaction_idx: usize, domain_idx: usize, cell_idx: usize, isotope_idx: usize, energy_group: usize) -> T {
    todo!()
}

pub fn weighted_macroscopic_cross_section<T: Float>(mcco: &MonteCarlo<T>, task_idx: usize, domain_idx: usize, cell_idx: usize, energy_group: usize) -> T {
    todo!()
}