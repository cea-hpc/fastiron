//! Super-structure used to store the problem's data
//!
//! This module contains the code of the super-structure holding all the
//! problem's data. The content of module [`crate::init`] may be moved
//! here in the future.

use std::fmt::Debug;

use num::zero;

use crate::constants::CustomFloat;
use crate::data::material_database::MaterialDatabase;
use crate::data::nuclear_data::NuclearData;
use crate::data::tallies::Tallies;
use crate::geometry::mc_domain::MCDomain;
use crate::parameters::Parameters;
use crate::particles::mc_particle::MCParticle;
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
#[derive(Debug)]
pub struct MonteCarloUnit<T: CustomFloat> {
    /// List of spatial domains.
    pub domain: Vec<MCDomain<T>>,
    /// Object storing all tallies of the simulation.
    pub tallies: Tallies<T>,
    /// Container for the timers used for performance measurements.
    pub fast_timer: MCFastTimerContainer,
    /// Weight of the particles at creation in a source zone
    pub source_particle_weight: T,
}

impl<T: CustomFloat> MonteCarloUnit<T> {
    /// Constructor.
    pub fn new(params: &Parameters<T>) -> Self {
        let tallies: Tallies<T> = Tallies::new(
            params.simulation_params.energy_spectrum.to_owned(),
            params.simulation_params.n_groups,
        );
        let fast_timer: MCFastTimerContainer = MCFastTimerContainer::default();

        Self {
            domain: Default::default(),
            tallies,
            fast_timer,
            source_particle_weight: zero(),
        }
    }

    /// Clear the cross section cache for each domain.
    pub fn clear_cross_section_cache(&mut self) {
        self.domain.iter_mut().for_each(|dd| {
            dd.clear_cross_section_cache();
        })
    }

    /// Update the energy spectrum by going over all the currently valid particles.
    pub fn update_spectrum(
        &mut self,
        container: &mut ParticleContainer<T>,
        mcdata: &MonteCarloData<T>,
    ) {
        if self.tallies.spectrum.file_name.is_empty() {
            return;
        }

        let update_function = |particle_list: &mut [MCParticle<T>], spectrum: &mut [u64]| {
            particle_list.iter_mut().for_each(|particle| {
                // load particle & update energy group
                particle.energy_group = mcdata
                    .nuclear_data
                    .get_energy_groups(particle.kinetic_energy);
                spectrum[particle.energy_group] += 1;
            });
        };

        // Iterate on processing particles
        update_function(
            &mut container.processing_particles,
            &mut self.tallies.spectrum.census_energy_spectrum,
        );
        // Iterate on processed particles
        update_function(
            &mut container.processed_particles,
            &mut self.tallies.spectrum.census_energy_spectrum,
        );
    }
}
