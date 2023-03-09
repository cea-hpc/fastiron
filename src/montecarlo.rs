use std::fmt::Display;

use num::{zero, Float, FromPrimitive, ToPrimitive};

use crate::material_database::MaterialDatabase;
use crate::mc::mc_fast_timer::MCFastTimerContainer;
use crate::mc::{
    mc_domain::MCDomain, mc_particle_buffer::MCParticleBuffer, mc_processor_info::MCProcessorInfo,
    mc_time_info::MCTimeInfo,
};
use crate::nuclear_data::NuclearData;
use crate::parameters::Parameters;
use crate::particle_vault_container::ParticleVaultContainer;
use crate::tallies::Tallies;

/// Super-structure used to contain all the problem's objects and data.
#[derive(Debug)]
pub struct MonteCarlo<T: Float + FromPrimitive> {
    /// List of spatial domains
    pub domain: Vec<MCDomain<T>>,
    /// Parameters of the problem
    pub params: Parameters,
    /// Object storing all data related to particles
    pub nuclear_data: NuclearData<T>,
    /// Container for all the particle vaults used during simulation
    pub particle_vault_container: ParticleVaultContainer<T>,
    /// Object storing all data related to materials
    pub material_database: MaterialDatabase<T>,
    /// Object storing all tallies of the simulation
    pub tallies: Tallies<T>,
    /// Object storing data related to the advancement of the simulation
    pub time_info: MCTimeInfo,
    /// Container for the timers used for performance measurements
    pub fast_timer: MCFastTimerContainer,
    /// Object storing data related to the processor and execution mode
    pub processor_info: MCProcessorInfo,
    /// Buffer used for potential spatial multithreading
    pub particle_buffer: MCParticleBuffer<T>,
    /// Weight of the particles at creation in a source zone
    pub source_particle_weight: f64,
}

impl<T: Float + FromPrimitive + Display + Default> MonteCarlo<T> {
    /// Constructor
    pub fn new(params: Parameters) -> Self {
        let tallies: Tallies<T> = Tallies::new(
            params.simulation_params.balance_tally_replications,
            params.simulation_params.flux_tally_replications,
            params.simulation_params.cell_tally_replications,
            params.simulation_params.energy_spectrum.to_owned(),
            params.simulation_params.n_groups,
        );
        let processor_info = MCProcessorInfo::new();
        let time_info: MCTimeInfo = MCTimeInfo::default();
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
}
