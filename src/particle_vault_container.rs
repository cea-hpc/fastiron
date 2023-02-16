use crate::{particle_vault::ParticleVault, send_queue::SendQueue};

/// Container for ParticleVaults. Can be used to define chunks
/// according to the user's specification. These chunks can be used
/// for further parallelization.
#[derive(Debug)]
pub struct ParticleVaultContainer {
    vault_size: u64,
    num_extra_vaults: u64,
    extra_vault_index: usize,
    send_queue: SendQueue,
    processing_vaults: Vec<ParticleVault>,
    processed_vaults: Vec<ParticleVault>,
    extra_vaults: Vec<ParticleVault>,
}
