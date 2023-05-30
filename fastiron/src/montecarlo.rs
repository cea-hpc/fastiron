//! Super-structure used to store the problem's data
//!
//! This module contains the code of the super-structure holding all the
//! problem's data. The content of module [`crate::init`] may be moved
//! here in the future.

use std::fmt::Debug;

use dashmap::DashMap;
use num::zero;

use crate::constants::CustomFloat;
use crate::data::material_database::MaterialDatabase;
use crate::data::nuclear_data::NuclearData;
use crate::data::tallies::Tallies;
use crate::geometry::mc_domain::MCDomain;
use crate::parameters::Parameters;
use crate::simulation::macro_cross_section::weighted_macroscopic_cross_section;
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
#[derive(Debug)]
pub struct MonteCarloUnit<T: CustomFloat> {
    /// List of spatial domains.
    pub domain: Vec<MCDomain<T>>,
    /// Object storing all tallies of the simulation.
    pub tallies: Tallies<T>,
    /// Object storing all tallies of the simulation.
    pub fast_timer: MCFastTimerContainer,
    /// Weight of the particles at creation in a source zone.
    pub unit_weight: T,
    /// HashMap used to lazily compute cross-sections.
    pub xs_cache: DashMap<(usize, usize, usize), T>,
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
            unit_weight: zero(),
            xs_cache: DashMap::default(),
        }
    }

    /// Clear the cross section cache for each domain.
    pub fn clear_cross_section_cache(&mut self) {
        self.domain.iter_mut().for_each(|dd| {
            dd.clear_cross_section_cache();
        });
        self.xs_cache.clear();
    }

    pub fn get_cross_section(
        &self,
        key: (usize, usize, usize),
        mcdata: &MonteCarloData<T>, // only needed when xs isn't computed
    ) -> T {
        *self.xs_cache.entry(key).or_insert_with(|| {
            let (domain, cell, energy_group) = key;
            let mat_gid: usize = self.domain[domain].cell_state[cell].material;
            let cell_nb_density: T = self.domain[domain].cell_state[cell].cell_number_density;
            weighted_macroscopic_cross_section(mcdata, mat_gid, cell_nb_density, energy_group)
        })
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
            .iter()
            .map(|dom| {
                dom.cell_state
                    .iter()
                    .map(|cell| {
                        // constant because cell volume is constant in our program
                        let cell_weight: T = cell.volume
                            * source_rate[cell.material]
                            * mcdata.params.simulation_params.dt;
                        cell_weight
                    })
                    .sum::<T>()
            })
            .sum();
    }
}
