use num::Float;

use crate::{particle_vault::ParticleVault, send_queue::SendQueue, mc::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle}};

/// Container for ParticleVaults. Can be used to define chunks
/// according to the user's specification. These chunks can be used
/// for further parallelization.
#[derive(Debug)]
pub struct ParticleVaultContainer<T: Float> {
    /// Size of the [ParticleVault]. Fixed at runtime for each run.
    pub vault_size: u64,
    /// Number of extra vaults needed. Fixed at runtime for each run.
    pub num_extra_vaults: u64,
    /// A running index for the number of particles in the extra
    /// particle vaults.
    pub extra_vault_index: usize,
    /// Stores particle index and neighbor index for any particles that hit
    /// TransitOffProcessor (See MCSubfacetAdjacencyEvent)
    pub send_queue: SendQueue,
    /// List of active particle vaults.
    pub processing_vaults: Vec<ParticleVault<T>>,
    /// List of census-ed particle vaults.
    pub processed_vaults: Vec<ParticleVault<T>>,
    /// List of extra particle vaults.
    pub extra_vaults: Vec<ParticleVault<T>>,
}

impl<T: Float> ParticleVaultContainer<T> {
    pub fn get_first_empty_processed_vault(&self) -> usize {
        todo!()
    }

    pub fn get_send_queue(&self) -> &SendQueue {
        todo!()
    }

    pub fn collapse_processing(&mut self) {
        todo!()
    }

    pub fn collapse_processed(&mut self) {
        todo!()
    }

    pub fn swap_processing_processed_vaults(&mut self) {
        todo!()
    }

    pub fn add_processing_particle(&mut self, particle: &MCBaseParticle<T>, fill_vault_index: &usize) {
        todo!()
    }

    pub fn add_extra_particle(&mut self, particle: &MCParticle<T>) {
        todo!()
    }

    pub fn clean_extra_vaults(&mut self) {
        todo!()
    }
}