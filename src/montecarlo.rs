use num::Float;

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
pub struct MonteCarlo<T: Float> {
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
    pub time_info: MCTimeInfo<T>,
    /// Container for the timers used for performance measurements
    pub fast_timer: MCFastTimerContainer,
    /// Object storing data related to the processor and execution mode
    pub processor_info: MCProcessorInfo,
    /// Buffer used for potential spatial multithreading
    pub particle_buffer: MCParticleBuffer<T>,
    /// Weight of the particles at creation in a source zone
    pub source_particle_weight: f64,
}

impl<T: Float> MonteCarlo<T> {
    /// Constructor
    pub fn new(params: &Parameters) -> Self {
        todo!()
    }

    /// Clear the cross section cache for each domain.
    pub fn clear_cross_section_cache(&mut self) {
        todo!()
    }
}
