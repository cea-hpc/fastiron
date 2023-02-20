use std::marker::PhantomData;

use num::Float;

use crate::montecarlo::MonteCarlo;

/// Structure used to represent the energy spectrum
/// of the problem, i.e. the distribution of particles
/// among energy levels.
#[derive(Debug)]
pub struct EnergySpectrum<T: Float> {
    float_type: PhantomData<T>,
    file_name: String,
    census_energy_spectrum: Vec<u64>,
}

impl<T: Float> EnergySpectrum<T> {
    /// Update its fields using the [MonteCarlo] Object.
    pub fn update_spectrum(monte_carlo: &MonteCarlo<T>) {
        todo!()
    }

    /// Print the spectrum.
    pub fn print_spectrum(monte_carlo: &MonteCarlo<T>) {
        todo!()
    }
}
