use std::fmt::{Debug, Display};
use std::io::Write;
use std::{fs::File, marker::PhantomData};

use num::{Float, FromPrimitive};

use crate::{mc::mc_utils::load_particle, montecarlo::MonteCarlo};

/// Structure used to represent the energy spectrum
/// of the problem, i.e. the distribution of particles
/// among energy levels.
#[derive(Debug)]
pub struct EnergySpectrum<T: Float> {
    float_type: PhantomData<T>,
    pub file_name: String,
    pub census_energy_spectrum: Vec<u64>,
}

impl<T: Float + Display + FromPrimitive + Default> EnergySpectrum<T> {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            float_type: Default::default(),
            file_name: name,
            census_energy_spectrum: Vec::with_capacity(size),
        }
    }

    /// Update its fields using the [MonteCarlo] Object.
    /// REPLACED BY EPONYMOUS FUNCTION OF MCCO
    pub fn update_spectrum(&mut self, mcco: &MonteCarlo<T>) {
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
                    let pp = load_particle(mcco, vv, particle_idx);
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
                    let pp = load_particle(mcco, vv, particle_idx);
                    self.census_energy_spectrum[pp.energy_group] += 1;
                });
            });
    }

    /// Print the spectrum.
    pub fn print_spectrum(&self, mcco: &MonteCarlo<T>) {
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
