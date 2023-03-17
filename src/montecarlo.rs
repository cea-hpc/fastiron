use std::fmt::Debug;

use num::zero;

use crate::constants::CustomFloat;
use crate::material_database::MaterialDatabase;
use crate::mc::mc_base_particle::MCBaseParticle;
use crate::mc::mc_fast_timer::{self, MCFastTimerContainer, Section};
use crate::mc::mc_utils::load_particle;
use crate::mc::{
    mc_domain::MCDomain, mc_particle_buffer::MCParticleBuffer, mc_processor_info::MCProcessorInfo,
    mc_time_info::MCTimeInfo,
};
use crate::nuclear_data::NuclearData;
use crate::parameters::Parameters;
use crate::particle_vault::ParticleVault;
use crate::particle_vault_container::ParticleVaultContainer;
use crate::tallies::Tallies;

/// Super-structure used to contain all the problem's objects and data.
#[derive(Debug)]
pub struct MonteCarlo<T: CustomFloat> {
    /// List of spatial domains
    pub domain: Vec<MCDomain<T>>,
    /// Parameters of the problem
    pub params: Parameters<T>,
    /// Object storing all data related to particles
    pub nuclear_data: NuclearData<T>,
    /// Container for all the particle vaults used during simulation
    pub particle_vault_container: ParticleVaultContainer<T>,
    /// Object storing all data related to materials
    pub material_database: MaterialDatabase<T>,
    /// Object storing all tallies of the simulation
    pub tallies: Tallies<T>,
    /// Object storing data related to the advancement of the simulation
    pub time_info: MCTimeInfo<T>,
    /// Container for the timers used for performance measurements
    pub fast_timer: MCFastTimerContainer,
    /// Object storing data related to the processor and execution mode
    pub processor_info: MCProcessorInfo,
    /// Buffer used for potential spatial multithreading
    pub particle_buffer: MCParticleBuffer<T>,
    /// Weight of the particles at creation in a source zone
    pub source_particle_weight: T,
}

impl<T: CustomFloat> MonteCarlo<T> {
    /// Constructor
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
            particle_buffer: Default::default(),
            source_particle_weight: zero(),
        }
    }

    /// Clear the cross section cache for each domain.
    pub fn clear_cross_section_cache(&mut self) {
        self.domain.iter_mut().for_each(|dd| {
            dd.clear_cross_section_cache();
        })
    }

    pub fn read_buffers(&mut self, fill_vault: &mut usize) {
        self.particle_buffer.buffers.iter().for_each(|b| {
            b.iter().for_each(|particle| {
                self.particle_vault_container
                    .add_processing_particle(MCBaseParticle::new(particle), fill_vault)
            })
        });
        self.particle_buffer.clear()
    }

    pub fn update_spectrum(&mut self) {
        if self.tallies.spectrum.file_name.is_empty() {
            println!("No output name specified for energy");
            return;
        }

        let update_function = |vault: &ParticleVault<T>, spectrum: &mut [u64]| {
            // We need to iterate on the index in order to access all particles, even invalid ones
            (0..vault.size()).into_iter().for_each(|particle_idx| {
                // load particle & update energy group
                let mut pp = load_particle(vault, particle_idx, self.time_info.time_step).unwrap();
                pp.energy_group = self.nuclear_data.get_energy_groups(pp.kinetic_energy);
                spectrum[pp.energy_group] += 1;
            });
        };

        // Check energy levels on processing particles
        // Iterate on processing vaults
        for vv in &self.particle_vault_container.processed_vaults {
            update_function(vv, &mut self.tallies.spectrum.census_energy_spectrum);
        }
        // Iterate on processed vaults
        self.particle_vault_container
            .processed_vaults
            .iter()
            .for_each(|vv| {
                update_function(vv, &mut self.tallies.spectrum.census_energy_spectrum);
            });
    }

    pub fn cycle_finalize(&mut self) {
        self.tallies.sum_tasks();

        mc_fast_timer::stop(self, Section::CycleFinalize);
        self.tallies.print_summary(self);
        mc_fast_timer::start(self, Section::CycleFinalize);

        self.tallies
            .balance_cumulative
            .add(&self.tallies.balance_task[0]);

        let new_start: u64 = self.tallies.balance_task[0].end;
        (0..self.tallies.balance_task.len())
            .into_iter()
            .for_each(|balance_idx| {
                self.tallies.balance_task[balance_idx].reset();
            });
        self.tallies.balance_task[0].start = new_start;

        (0..self.tallies.scalar_flux_domain.len())
            .into_iter()
            .for_each(|domain_idx| {
                // Sum on replicated cell tallies and resets them
                (1..self.tallies.num_cell_tally_replications)
                    .into_iter()
                    .for_each(|rep_idx| {
                        let val = self.tallies.cell_tally_domain[domain_idx].task[rep_idx as usize]
                            .clone(); // is there a cheaper way?
                        self.tallies.cell_tally_domain[domain_idx].task[0].add(&val);
                        self.tallies.cell_tally_domain[domain_idx].task[rep_idx as usize].reset();
                    });

                // Sum on replciated scalar flux tallies and resets them
                (1..self.tallies.num_flux_replications)
                    .into_iter()
                    .for_each(|rep_idx| {
                        let val = self.tallies.scalar_flux_domain[domain_idx].task
                            [rep_idx as usize]
                            .clone(); // is there a cheaper way?
                        self.tallies.scalar_flux_domain[domain_idx].task[0].add(&val);
                        self.tallies.scalar_flux_domain[domain_idx].task[rep_idx as usize].reset();
                    });

                if self.params.simulation_params.coral_benchmark {
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
