use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

use crate::constants::CustomFloat;
use crate::montecarlo::MonteCarlo;

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

    /// Print the spectrum.
    pub fn print_spectrum<T: CustomFloat>(&self, mcco: &MonteCarlo<T>) {
        if self.file_name.is_empty() {
            return;
        }
        let levels = mcco.nuclear_data.energies.len();
        let mut path = self.file_name.to_owned();
        path.push_str(".dat");
        let mut file = File::create(path).unwrap();

        writeln!(
            file,
            "energy level index |         energy level |         count"
        )
        .unwrap();

        writeln!(
            file,
            "-------------------|----------------------|--------------"
        )
        .unwrap();

        (0..levels).for_each(|ii| {
            writeln!(
                file,
                "{:>18} | {:>20.15} | {:>13}",
                ii, mcco.nuclear_data.energies[ii], self.census_energy_spectrum[ii]
            )
            .unwrap();
        })
    }
}
