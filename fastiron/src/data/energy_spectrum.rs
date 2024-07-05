//! Discrete energy spectrum
//!
//! This module contains the structure used to store and build a discrete
//! energy spectrum from the problem's population.

use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

use crate::constants::CustomFloat;
use crate::montecarlo::MonteCarloData;

/// Structure used to represent the energy spectrum
/// of the problem, i.e. the distribution of particles
/// among energy levels.
///
/// The spectrum is simply done by sorting particles into energy groups and
/// counting the number of particles in each group. Note that the entire range of
/// values is separated in segments using a logarithmic scale.\
/// The spectrum is printed as a MarkDown table if an output name file is specified
/// at launch.
#[derive(Debug)]
pub struct EnergySpectrum {
    /// Name of the output file.
    pub file_name: String,
    /// Population of the energy groups i.e. count of particle in each one.
    pub census_energy_spectrum: Vec<u64>,
}

impl EnergySpectrum {
    /// Constructor.
    pub fn new(name: String, size: usize) -> Self {
        Self {
            file_name: name,
            census_energy_spectrum: vec![0; size + 1],
        }
    }

    /// Print the spectrum. This function does nothing if no output file were
    /// specified at launch.
    pub fn print_spectrum<T: CustomFloat>(&self, mcdata: &MonteCarloData<T>) {
        if self.file_name.is_empty() {
            return;
        }
        let levels = mcdata.nuclear_data.energies.len();
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
                ii, mcdata.nuclear_data.energies[ii], self.census_energy_spectrum[ii]
            )
            .unwrap();
        })
    }
}
