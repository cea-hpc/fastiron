//! Super-structure used to store the problem's data
//!
//! This module contains the code of the super-structure holding all the
//! problem's data. The content of module [`crate::init`] may be moved
//! here in the future.

use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use atomic::{Atomic, Ordering};
use num::zero;

use crate::constants::CustomFloat;
use crate::data::energy_spectrum::EnergySpectrum;
use crate::data::material_database::MaterialDatabase;
use crate::data::nuclear_data::NuclearData;
use crate::data::tallies::{Balance, FluenceDomain, Tallies};
use crate::geometry::mc_domain::MCDomain;
use crate::parameters::{BenchType, Parameters};
use crate::particles::particle_collection::ParticleCollection;
use crate::particles::particle_container::ParticleContainer;
use crate::utils::mc_fast_timer::MCFastTimerContainer;
use crate::utils::mc_processor_info::MCProcessorInfo;

/// Super-structure used to contain all the problem's data.
///
/// This structure is used to store read-only data, i.e. after initialization,
/// its content does not change until the end of the simulation.
#[derive(Debug, Default)]
pub struct MonteCarloData<T: CustomFloat> {
    /// Parameters of the problem.
    pub params: Parameters<T>,
    /// Object storing all data related to particles.
    pub nuclear_data: NuclearData<T>,
    /// Object storing all data related to materials.
    pub material_database: MaterialDatabase<T>,
    /// Object storing data related to the processor and execution mode.
    pub exec_info: MCProcessorInfo,
    /// Weight of a particle to be spawned. This is a constant in our case but
    /// isn't in more flexible simulation.
    pub source_particle_weight: T,
    /// Current total number of particles in the simulation. This value is updated at
    /// each cycle for ease of access by all [MonteCarloUnit].
    pub global_n_particles: usize,
}

impl<T: CustomFloat> MonteCarloData<T> {
    /// Constructor.
    pub fn new(params: Parameters<T>) -> Self {
        let exec_info = MCProcessorInfo::new(&params.simulation_params);

        Self {
            params,
            exec_info,
            ..Default::default()
        }
    }
}

/// Super-structure used to contain unit-specific data of the Monte-Carlo problem.
/// The notion of unit is specified ....
#[derive(Debug, Default)]
pub struct MonteCarloUnit<T: CustomFloat> {
    /// List of spatial domains.
    pub domain: MCDomain<T>,
    /// Object storing all tallies of the simulation.
    pub tallies: Tallies<T>,
    /// Object storing all tallies of the simulation.
    pub fast_timer: MCFastTimerContainer,
    /// Weight of the particles at creation in a source zone.
    pub unit_weight: T,
    /// HashMap used to lazily compute cross-sections.
    pub xs_cache: XSCache<T>,
}

impl<T: CustomFloat> MonteCarloUnit<T> {
    /// Clear the cross section cache for each domain.
    pub fn clear_cross_section_cache(&mut self) {
        self.xs_cache
            .cache
            .iter_mut()
            .for_each(|xs| xs.store(zero(), Ordering::Relaxed))
    }

    pub fn update_unit_weight(&mut self, mcdata: &MonteCarloData<T>) {
        let source_rate: Vec<T> = mcdata
            .material_database
            .mat
            .iter()
            .map(|mat| mcdata.params.material_params[&mat.name].source_rate)
            .collect();

        self.unit_weight = self
            .domain
            .cell_state
            .iter()
            .map(|cell| {
                // constant because cell volume is constant in our program
                let cell_weight: T =
                    cell.volume * source_rate[cell.material] * mcdata.params.simulation_params.dt;
                cell_weight
            })
            .sum::<T>()
    }
}

#[derive(Debug, Default)]
pub struct XSCache<T: CustomFloat> {
    pub num_groups: usize,
    pub cache: Vec<Atomic<T>>,
}

// maybe make theses accesses unchecked?
impl<T: CustomFloat> Index<(usize, usize)> for XSCache<T> {
    type Output = Atomic<T>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cache[index.0 * self.num_groups + index.1]
    }
}

impl<T: CustomFloat> IndexMut<(usize, usize)> for XSCache<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cache[index.0 * self.num_groups + index.1]
    }
}

pub struct MonteCarloResults<T: CustomFloat> {
    /// Balance used for cumulative and centralized statistics.
    pub balance_cumulative: Balance,
    /// Top-level structure used to compute fluence data.
    pub fluence: FluenceDomain<T>,
    /// Energy spectrum of the problem.
    pub spectrum: EnergySpectrum,
    /// Enum used to ...
    pub bench_type: BenchType,
}

impl<T: CustomFloat> MonteCarloResults<T> {
    pub fn new(spectrum_name: String, spectrum_size: usize, bench_type: BenchType) -> Self {
        Self {
            balance_cumulative: Default::default(),
            fluence: Default::default(),
            spectrum: EnergySpectrum::new(spectrum_name, spectrum_size),
            bench_type,
        }
    }

    /// Update the energy spectrum by going over all the currently valid particles.
    pub fn update_spectrum(&mut self, containers: &[ParticleContainer<T>]) {
        if self.spectrum.file_name.is_empty() {
            return;
        }

        let update_function = |particle_list: &ParticleCollection<T>, spectrum: &mut [u64]| {
            particle_list.into_iter().for_each(|particle| {
                spectrum[particle.energy_group] += 1;
            });
        };

        // Iterate on all containers
        containers.iter().for_each(|container| {
            // Iterate on processing particles
            update_function(
                &container.processing_particles,
                &mut self.spectrum.census_energy_spectrum,
            );
            // Iterate on processed particles
            update_function(
                &container.processed_particles,
                &mut self.spectrum.census_energy_spectrum,
            );
        })
    }

    pub fn update_stats(&mut self, mcunits: &mut [MonteCarloUnit<T>]) {
        mcunits.iter_mut().for_each(|mcunit| {
            self.balance_cumulative
                .add_to_self(&mcunit.tallies.balance_cycle);
            if self.bench_type != BenchType::Standard {
                self.fluence.compute(&mcunit.tallies.scalar_flux_domain)
            }
            mcunit.tallies.cycle_finalize();
        })
    }
}
