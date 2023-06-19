//! Code for cross-section computation a.k.a probability density computation
//!
//! This module contains function used to compute cross-section data from known
//! physical quantities of the material and reacting isotope.

use num::{zero, FromPrimitive};

use crate::{constants::CustomFloat, montecarlo::MonteCarloData};

/// Computes the total number-density-weighted macroscopic
/// cross section in the cell.
///
/// Note that this function is isotope-specific; However the proxy-app
/// only accounts for simulation of a single isotope type.
fn macroscopic_total_cross_section<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mat_gid: usize,
    isotope_idx: usize,
    cell_nb_density: T,
    energy_group: usize,
) -> T {
    let atom_fraction: T = mcdata.material_database.mat[mat_gid].iso[isotope_idx].atom_fraction;

    if (atom_fraction == zero()) | (cell_nb_density == zero()) {
        // one of the two is 0
        let res: T = FromPrimitive::from_f64(1e-20).unwrap();
        return res;
    }

    let isotope_gid = mcdata.material_database.mat[mat_gid].iso[isotope_idx].gid;
    let micro_cross_section: T = mcdata
        .nuclear_data
        .get_total_cross_section(isotope_gid, energy_group);

    atom_fraction * cell_nb_density * micro_cross_section
}

/// Computes the number-density-weighted macroscopic cross section
/// of a collection of isotopes in the cell.
///
/// Note that there is not really any weighting, this comes from the original
/// choice of Quicksilver to leave material weighting out of the proxy-app.
pub fn weighted_macroscopic_cross_section<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mat_gid: usize,
    cell_nb_density: T,
    energy_group: usize,
) -> T {
    let mut sum: T = zero();
    let n_isotopes: usize = mcdata.material_database.mat[mat_gid].iso.len();

    (0..n_isotopes).for_each(|isotope_idx| {
        sum += macroscopic_total_cross_section(
            mcdata,
            mat_gid,
            isotope_idx,
            cell_nb_density,
            energy_group,
        )
    });

    sum
}
