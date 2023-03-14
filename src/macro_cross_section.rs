use num::{zero, FromPrimitive};

use crate::{montecarlo::MonteCarlo, constants::{physical::TINY_FLOAT, CustomFloat}};

/// Computes the reaction-specific number-density-weighted
/// macroscopic cross section of a cell.
pub fn macroscopic_cross_section<T: CustomFloat>(
    mcco: &MonteCarlo<T>,
    reaction_idx: usize,
    domain_idx: usize,
    cell_idx: usize,
    isotope_idx: usize,
    energy_group: usize,
) -> T {
    let global_mat_idx = mcco.domain[domain_idx].cell_state[cell_idx].material;

    let atom_fraction: T =
        mcco.material_database.mat[global_mat_idx].iso[isotope_idx].atom_fraction;
    let cell_number_density: T = mcco.domain[domain_idx].cell_state[cell_idx].cell_number_density;

    let threshold: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
    // comparison to 0 might be possible since it means it's not initialized & it's not a result of a computation?
    if (atom_fraction < threshold) | (cell_number_density < threshold) {
        // one of the two is 0
        let res: T = FromPrimitive::from_f64(1e-20).unwrap();
        return res;
    }

    let isotope_gid = mcco.material_database.mat[domain_idx].iso[isotope_idx].gid;
    let micro_cross_section: T =
        mcco.nuclear_data
            .get_reaction_cross_section(reaction_idx, isotope_gid, energy_group);

    atom_fraction * cell_number_density * micro_cross_section
}

/// Computes the total number-density-weighted macroscopic
/// cross section of a cell. This additional method replaces
/// the use of a magic value (-1) for `reaction_idx`.
pub fn macroscopic_total_cross_section<T: CustomFloat>(
    mcco: &MonteCarlo<T>,
    domain_idx: usize,
    cell_idx: usize,
    isotope_idx: usize,
    energy_group: usize,
) -> T {
    let global_mat_idx = mcco.domain[domain_idx].cell_state[cell_idx].material;

    let atom_fraction: T =
        mcco.material_database.mat[global_mat_idx].iso[isotope_idx].atom_fraction;
    let cell_number_density: T = mcco.domain[domain_idx].cell_state[cell_idx].cell_number_density;

    let threshold: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
    // comparison to 0 might be possible since it means it's not initialized & it's not a result of a computation?
    if (atom_fraction < threshold) | (cell_number_density < threshold) {
        // one of the two is 0
        let res: T = FromPrimitive::from_f64(1e-20).unwrap();
        return res;
    }

    let isotope_gid = mcco.material_database.mat[domain_idx].iso[isotope_idx].gid;
    let micro_cross_section: T = mcco
        .nuclear_data
        .get_total_cross_section(isotope_gid, energy_group);

    atom_fraction * cell_number_density * micro_cross_section
}

/// Computes the number-density-weighted macroscopic cross section
/// of the collection of isotopes in a cell.
pub fn weighted_macroscopic_cross_section<T: CustomFloat>(
    mcco: &mut MonteCarlo<T>,
    domain_idx: usize,
    cell_idx: usize,
    energy_group: usize,
) -> T {
    // early return
    let precomputed_cross_section =
        mcco.domain[domain_idx].cell_state[cell_idx].total[energy_group];
    if precomputed_cross_section > zero() {
        return precomputed_cross_section;
    }

    let mut sum: T = zero();
    let global_material_idx: usize = mcco.domain[domain_idx].cell_state[cell_idx].material;
    let n_isotopes: usize = mcco.material_database.mat[global_material_idx].iso.len();

    (0..n_isotopes).into_iter().for_each(|isotope_idx| {
        sum = sum
            + macroscopic_total_cross_section(mcco, domain_idx, cell_idx, isotope_idx, energy_group)
    });

    // atomic in original code
    mcco.domain[domain_idx].cell_state[cell_idx].total[energy_group] = sum;

    sum
}
