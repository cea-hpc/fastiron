use crate::material_database::MaterialDatabase;
use crate::mc::{
    mc_domain::MCDomain, mc_fast_timer::MCFastTimer, mc_particle_buffer::MCParticleBuffer,
    mc_processor_info::MCProcessorInfo, mc_time_info::MCTimeInfo,
};
use crate::nuclear_data::NuclearData;
use crate::parameters::Parameters;
use crate::particle_vault_container::ParticleVaultContainer;
use crate::tallies::Tallies;

#[derive(Debug)]
pub struct MonteCarlo {
    pub domain: Vec<MCDomain>,

    pub params: Parameters,
    pub nuclear_data: NuclearData,
    pub particle_vault_container: ParticleVaultContainer,
    pub material_database: MaterialDatabase,
    pub tallies: Tallies,
    pub time_info: MCTimeInfo,
    pub fast_timer: MCFastTimer,
    pub processor_info: MCProcessorInfo,
    pub particle_buffer: MCParticleBuffer,

    pub source_particle_weight: f64,
}

impl MonteCarlo {
    /// Constructor
    pub fn new(params: &Parameters) -> Self {
        todo!()
    }
}

impl MonteCarlo {
    /// Clear the cross section cache for each domain.
    pub fn clear_cross_section_cache(&mut self) {
        todo!()
    }
}