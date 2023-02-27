use num::{Float, zero, FromPrimitive};

use crate::montecarlo::MonteCarlo;

/// Computes the reaction-specific number-density-weighted 
/// macroscopic cross section of a cell.
pub fn macroscopic_cross_section<T: Float + FromPrimitive>(
    mcco: &MonteCarlo<T>,
    reaction_idx: usize,
    domain_idx: usize,
    cell_idx: usize,
    isotope_idx: usize,
    energy_group: usize,
) -> T {
    let global_mat_idx = mcco.domain[domain_idx].cell_state[cell_idx].material;

    let atom_fraction: T = mcco.material_database.mat[global_mat_idx].iso[isotope_idx].atom_fraction;
    let cell_number_density: T = mcco.domain[domain_idx].cell_state[cell_idx].cell_number_density;
    
    // Early return; Change these to use a threshold
    if (atom_fraction == zero()) | (cell_number_density == zero()) {
        let res: T = FromPrimitive::from_f64(1e-20).unwrap();
        return res;
    }

    let isotope_gid = mcco.material_database.mat[domain_idx].iso[isotope_idx].gid;
    let micro_cross_section: T = mcco.nuclear_data.get_reaction_cross_section(reaction_idx, isotope_gid, energy_group);

    atom_fraction * cell_number_density * micro_cross_section
}

/// Computes the total number-density-weighted macroscopic 
/// cross section of a cell. This additional method replaces 
/// the use of a magic value (-1) for `reaction_idx`.
pub fn macroscopic_total_cross_section<T: Float + FromPrimitive>(
    mcco: &MonteCarlo<T>,
    domain_idx: usize,
    cell_idx: usize,
    isotope_idx: usize,
    energy_group: usize,
) -> T {
    let global_mat_idx = mcco.domain[domain_idx].cell_state[cell_idx].material;

    let atom_fraction: T = mcco.material_database.mat[global_mat_idx].iso[isotope_idx].atom_fraction;
    let cell_number_density: T = mcco.domain[domain_idx].cell_state[cell_idx].cell_number_density;
    
    // Early return; Change these to use a threshold
    if (atom_fraction == zero()) | (cell_number_density == zero()) {
        let res: T = FromPrimitive::from_f64(1e-20).unwrap();
        return res;
    }

    let isotope_gid = mcco.material_database.mat[domain_idx].iso[isotope_idx].gid;
    let micro_cross_section: T = mcco.nuclear_data.get_total_cross_section(isotope_gid, energy_group);

    atom_fraction * cell_number_density * micro_cross_section
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
