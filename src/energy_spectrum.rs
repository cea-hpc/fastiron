use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

use crate::constants::CustomFloat;
use crate::{mc::mc_utils::load_particle, montecarlo::MonteCarlo};

/// Structure used to represent the energy spectrum
/// of the problem, i.e. the distribution of particles
/// among energy levels.
#[derive(Debug)]
pub struct EnergySpectrum {
    pub file_name: String,
    pub census_energy_spectrum: Vec<u64>,
}

impl EnergySpectrum {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            file_name: name,
            census_energy_spectrum: vec![0; size + 1],
        }
    }

    /// Update its fields using the [MonteCarlo] Object.
    /// REPLACED BY EPONYMOUS FUNCTION OF MCCO
    pub fn update_spectrum<T: CustomFloat>(&mut self, mcco: &MonteCarlo<T>) {
        if self.file_name.is_empty() {
            return;
        }
        // Check energy levels on processing particles
        // Iterate on processing vaults
        mcco.particle_vault_container
            .processing_vaults
            .iter()
            .for_each(|vv| {
                // We need to iterate on the index in order to access all particles, even invalid ones
                (0..vv.size()).into_iter().for_each(|particle_idx| {
                    let mut pp = load_particle(vv, particle_idx, mcco.time_info.time_step).unwrap();
                    pp.energy_group = mcco.nuclear_data.get_energy_groups(pp.kinetic_energy);
                    self.census_energy_spectrum[pp.energy_group] += 1;
                });
            });
        // Iterate on processed vaults
        mcco.particle_vault_container
            .processed_vaults
            .iter()
            .for_each(|vv| {
                // We need to iterate on the index in order to access all particles, even invalid ones
                (0..vv.size()).into_iter().for_each(|particle_idx| {
                    let mut pp = load_particle(vv, particle_idx, mcco.time_info.time_step).unwrap();
                    pp.energy_group = mcco.nuclear_data.get_energy_groups(pp.kinetic_energy);
                    self.census_energy_spectrum[pp.energy_group] += 1;
                });
            });
    }

    /// Print the spectrum.
    pub fn print_spectrum<T: CustomFloat>(&self, mcco: &MonteCarlo<T>) {
        if self.file_name.is_empty() {
            return;
        }
        let levels = mcco.nuclear_data.energies.len();
        let mut path = self.file_name.to_owned();
        path.push_str(".dat");
        let mut file = File::create(path).unwrap();

        writeln!(file, "energy level index | energy level | count").unwrap();

        (0..levels).into_iter().for_each(|ii| {
            writeln!(
                file,
                "{}     {}     {}",
                ii, mcco.nuclear_data.energies[ii], self.census_energy_spectrum[ii]
            )
            .unwrap();
        })
    }
}
