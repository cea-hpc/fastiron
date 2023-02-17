use num::Float;

use crate::{particle_vault::ParticleVault, send_queue::SendQueue};

/// Container for ParticleVaults. Can be used to define chunks
/// according to the user's specification. These chunks can be used
/// for further parallelization.
#[derive(Debug)]
pub struct ParticleVaultContainer<T: Float> {
    /// Size of the [ParticleVault]. Fixed at runtime for each run.
    vault_size: u64,
    /// Number of extra vaults needed. Fixed at runtime for each run.
    num_extra_vaults: u64,
    /// A running index for the number of particles in the extra
    /// particle vaults.
    extra_vault_index: usize,
    /// Stores particle index and neighbor index for any particles that hit
    /// TransitOffProcessor (See MCSubfacetAdjacencyEvent)
    send_queue: SendQueue,
    /// List of active particle vaults.
    processing_vaults: Vec<ParticleVault<T>>,
    /// List of census-ed particle vaults.
    processed_vaults: Vec<ParticleVault<T>>,
    /// List of extra particle vaults.
    extra_vaults: Vec<ParticleVault<T>>,
}
