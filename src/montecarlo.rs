//! Super-structure used to store the problem's data
//!
//! This module contains the code of the super-structure holding all the
//! problem's data. The content of module [`crate::init_mc`] may be moved
//! here in the future.

use std::fmt::Debug;

use num::zero;

use crate::constants::CustomFloat;
use crate::data::material_database::MaterialDatabase;
use crate::data::nuclear_data::NuclearData;
use crate::data::tallies::Tallies;
use crate::geometry::mc_domain::MCDomain;
use crate::parameters::{BenchType, Parameters};
use crate::particles::load_particle::load_particle;
use crate::particles::particle_vault::ParticleVault;
use crate::particles::particle_vault_container::ParticleVaultContainer;
use crate::utils::mc_fast_timer::{self, MCFastTimerContainer, Section};
use crate::utils::mc_processor_info::MCProcessorInfo;
use crate::utils::mc_time_info::MCTimeInfo;

/// Super-structure used to contain all the problem's objects and data.
#[derive(Debug)]
pub struct MonteCarlo<T: CustomFloat> {
    /// List of spatial domains.
    pub domain: Vec<MCDomain<T>>,
    /// Parameters of the problem.
    pub params: Parameters<T>,
    /// Object storing all data related to particles.
    pub nuclear_data: NuclearData<T>,
    /// Container for all the particle vaults used during simulation.
    pub particle_vault_container: ParticleVaultContainer<T>,
    /// Object storing all data related to materials.
    pub material_database: MaterialDatabase<T>,
    /// Object storing all tallies of the simulation.
    pub tallies: Tallies<T>,
    /// Object storing data related to the advancement of the simulation.
    pub time_info: MCTimeInfo<T>,
    /// Container for the timers used for performance measurements.
    pub fast_timer: MCFastTimerContainer,
    /// Object storing data related to the processor and execution mode.
    pub processor_info: MCProcessorInfo,
    /// Weight of the particles at creation in a source zone
    pub source_particle_weight: T,
}

impl<T: CustomFloat> MonteCarlo<T> {
    /// Constructor.
    pub fn new(params: Parameters<T>) -> Self {
        let tallies: Tallies<T> = Tallies::new(
            params.simulation_params.balance_tally_replications,
            params.simulation_params.flux_tally_replications,
            params.simulation_params.cell_tally_replications,
            params.simulation_params.energy_spectrum.to_owned(),
            params.simulation_params.n_groups,
        );
        let processor_info = MCProcessorInfo::new();
        let time_info = MCTimeInfo::<T>::default();
        let fast_timer: MCFastTimerContainer = MCFastTimerContainer::default();

        let num_proc = processor_info.num_processors;
        let num_particles = params.simulation_params.n_particles as usize;
        let mut batch_size = params.simulation_params.batch_size as usize;
        let mut num_batches = params.simulation_params.n_batches as usize;

        let n_particles_per_process = num_particles / num_proc;

        if batch_size == 0 {
            batch_size = if n_particles_per_process % num_batches == 0 {
                n_particles_per_process / num_batches
            } else {
                n_particles_per_process / num_batches + 1
            }
        } else {
            num_batches = if n_particles_per_process % batch_size == 0 {
                n_particles_per_process / batch_size
            } else {
                n_particles_per_process / batch_size + 1
            }
        }
        assert_ne!(batch_size, 0);

        let mut vec_size: usize = 0;

        params.material_params.values().for_each(|mp| {
            let nb = params.cross_section_params[&mp.fission_cross_section]
                .nu_bar
                .ceil()
                .to_usize()
                .unwrap();
            if nb * batch_size > vec_size {
                vec_size = nb * batch_size;
            }
        });
        if vec_size == 0 {
            vec_size = 2 * batch_size;
        }

        let num_extra_vaults = (vec_size / batch_size) + 1;
        let particle_vault_container: ParticleVaultContainer<T> =
            ParticleVaultContainer::new(batch_size, num_batches, num_extra_vaults);

        Self {
            domain: Default::default(),
            params,
            nuclear_data: Default::default(),
            particle_vault_container,
            material_database: Default::default(),
            tallies,
            time_info,
            fast_timer,
            processor_info,
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
    pub fn update_spectrum(&mut self) {
        if self.tallies.spectrum.file_name.is_empty() {
            println!("No output name specified for energy");
            return;
        }

        let update_function = |vault: &ParticleVault<T>, spectrum: &mut [u64]| {
            (0..vault.size()).for_each(|particle_idx| {
                // load particle & update energy group
                let mut pp = load_particle(vault, particle_idx, self.time_info.time_step).unwrap();
                pp.energy_group = self.nuclear_data.get_energy_groups(pp.kinetic_energy);
                spectrum[pp.energy_group] += 1;
            });
        };

        // Iterate on processing vaults
        for vv in &self.particle_vault_container.processing_vaults {
            update_function(vv, &mut self.tallies.spectrum.census_energy_spectrum);
        }
        // Iterate on processed vaults
        for vv in &self.particle_vault_container.processed_vaults {
            update_function(vv, &mut self.tallies.spectrum.census_energy_spectrum);
        }
    }

    /// Print stats of the current cycle and update the cumulative counters.
    pub fn cycle_finalize(&mut self) {
        self.tallies.sum_tasks();

        mc_fast_timer::stop(self, Section::CycleFinalize);
        self.tallies.print_summary(self);
        mc_fast_timer::start(self, Section::CycleFinalize);

        self.tallies
            .balance_cumulative
            .add(&self.tallies.balance_task[0]);

        let new_start: u64 = self.tallies.balance_task[0].end;
        (0..self.tallies.balance_task.len()).for_each(|balance_idx| {
            self.tallies.balance_task[balance_idx].reset();
        });
        self.tallies.balance_task[0].start = new_start;

        (0..self.tallies.scalar_flux_domain.len()).for_each(|domain_idx| {
            // Sum on replicated cell tallies and resets them
            (1..self.tallies.num_cell_tally_replications).for_each(|rep_idx| {
                let val = self.tallies.cell_tally_domain[domain_idx].task[rep_idx as usize].clone(); // is there a cheaper way?
                self.tallies.cell_tally_domain[domain_idx].task[0].add(&val);
                self.tallies.cell_tally_domain[domain_idx].task[rep_idx as usize].reset();
            });

            // Sum on replciated scalar flux tallies and resets them
            (1..self.tallies.num_flux_replications).for_each(|rep_idx| {
                let val =
                    self.tallies.scalar_flux_domain[domain_idx].task[rep_idx as usize].clone(); // is there a cheaper way?
                self.tallies.scalar_flux_domain[domain_idx].task[0].add(&val);
                self.tallies.scalar_flux_domain[domain_idx].task[rep_idx as usize].reset();
            });

            if self.params.simulation_params.coral_benchmark != BenchType::Standard {
                self.tallies
                    .fluence
                    .compute(domain_idx, &self.tallies.scalar_flux_domain[domain_idx]);
            }
            self.tallies.cell_tally_domain[domain_idx].task[0].reset();
            self.tallies.scalar_flux_domain[domain_idx].task[0].reset();
        });
        self.update_spectrum();
    }
}
