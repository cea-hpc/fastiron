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

#[derive(Debug)]
pub struct MonteCarlo<T: Float> {
    pub domain: Vec<MCDomain<T>>,

    pub params: Parameters,
    pub nuclear_data: NuclearData<T>,
    pub particle_vault_container: ParticleVaultContainer<T>,
    pub material_database: MaterialDatabase<T>,
    pub tallies: Tallies<T>,
    pub time_info: MCTimeInfo<T>,
    pub fast_timer: MCFastTimerContainer,
    pub processor_info: MCProcessorInfo,
    pub particle_buffer: MCParticleBuffer<T>,

    pub source_particle_weight: f64,
}

impl<T: Float> MonteCarlo<T> {
    /// Constructor
    pub fn new(params: &Parameters) -> Self {
        todo!()
    }
}

impl<T: Float> MonteCarlo<T> {
    /// Clear the cross section cache for each domain.
    pub fn clear_cross_section_cache(&mut self) {
        todo!()
    }
}
