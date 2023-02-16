/// Structure used to represent the energy spectrum
/// of the problem, i.e. the distribution of particles
/// among energy levels.
#[derive(Debug)]
pub struct EnergySpectrum {
    file_name: String,
    census_energy_spectrum: Vec<u64>,
}
