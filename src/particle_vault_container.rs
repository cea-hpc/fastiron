use num::{Float, FromPrimitive};

use crate::{
    mc::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle},
    particle_vault::ParticleVault,
    send_queue::SendQueue,
};

/// Container for ParticleVaults.
#[derive(Debug)]
pub struct ParticleVaultContainer<T: Float> {
    /// Size of the [ParticleVault]. Fixed at runtime for each run.
    pub vault_size: usize,
    /// Number of extra vaults needed. Fixed at runtime for each run.
    pub num_extra_vaults: usize,
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
    pub extra_vault: Vec<ParticleVault<T>>,
}

impl<T: Float + FromPrimitive> ParticleVaultContainer<T> {
    pub fn new(vault_size: usize, num_vaults: usize, num_extra_vaults: usize) -> Self {
        let mut processing_vaults: Vec<ParticleVault<T>> =
            vec![ParticleVault::default(); num_vaults];
        let mut processed_vaults: Vec<ParticleVault<T>> =
            vec![ParticleVault::default(); num_vaults];
        (0..num_vaults).into_iter().for_each(|ii| {
            processing_vaults[ii].reserve(vault_size);
            processed_vaults[ii].reserve(vault_size);
        });
        let mut extra_vault: Vec<ParticleVault<T>> =
            vec![ParticleVault::default(); num_extra_vaults];
        extra_vault.iter_mut().for_each(|vv| vv.reserve(vault_size));
        let send_queue = SendQueue {
            data: Vec::with_capacity(vault_size),
        };
        Self {
            vault_size,
            num_extra_vaults,
            extra_vault_index: 0,
            send_queue,
            processing_vaults,
            processed_vaults,
            extra_vault,
        }
    }

    /// Returns the number of processing vaults
    pub fn processing_size(&self) -> usize {
        self.processing_vaults.len()
    }

    /// Returns the number of processed vaults
    pub fn processed_size(&self) -> usize {
        self.processed_vaults.len()
    }

    /// Returns the processing [ParticleVault] that is currently pointed to
    /// by the index. May be able to remove the mutable property depending
    /// on usage or even return the object directly.
    pub fn get_task_processing_vault(&mut self, index: usize) -> &mut ParticleVault<T> {
        &mut self.processing_vaults[index]
    }

    /// Returns the processed [ParticleVault] that is currently pointed to
    /// by the index. May be able to remove the mutable property depending
    /// on usage or even return the object directly.
    pub fn get_task_processed_vault(&mut self, index: usize) -> &mut ParticleVault<T> {
        &mut self.processed_vaults[index]
    }

    /// Returns the index of the first empty vault in among the processed vaults.
    /// Does the original function even work correctly?
    pub fn get_first_empty_processed_vault(&self) -> Option<usize> {
        (0..self.processed_vaults.len()).find(|&idx| self.processed_vaults[idx].empty())
    }

    /// Returns a reference to the internal [SendQueue] object.
    pub fn get_send_queue(&mut self) -> &mut SendQueue {
        &mut self.send_queue
    }

    /// Counts the total number of particles in processing vaults
    pub fn particles_processing_size(&self) -> usize {
        let mut total: usize = 0;

        self.processing_vaults
            .iter()
            .for_each(|vv| total += vv.size());

        total
    }

    /// Counts the total number of particles in processed vaults
    pub fn particles_processed_size(&self) -> usize {
        let mut total: usize = 0;

        self.processed_vaults
            .iter()
            .for_each(|vv| total += vv.size());

        total
    }

    /// Counts the total number of particles in extra vaults
    pub fn particles_extra_size(&self) -> usize {
        let mut total: usize = 0;

        self.extra_vault.iter().for_each(|vv| total += vv.size());

        total
    }

    /// Collapse the processing vaults in the lowest amount
    /// of vaults needed to hold them. Removes excess vaults.
    pub fn collapse_processing(&mut self) {
        let mut fill_vault_index: usize = 0;
        let mut from_vault_index: usize = self.processing_size().saturating_sub(1);

        while fill_vault_index < from_vault_index {
            if self.processing_vaults[fill_vault_index].size() == self.vault_size {
                fill_vault_index += 1;
            } else if self.processing_vaults[from_vault_index].size() == 0 {
                from_vault_index -= 1;
            } else {
                let fill_size = self.vault_size - self.processing_vaults[fill_vault_index].size();
                // ugly workaround, a more elegant solution might be possible
                let mut from_vault = self.processing_vaults[from_vault_index].clone();
                self.processing_vaults[fill_vault_index].collapse(fill_size, &mut from_vault);
                self.processing_vaults[from_vault_index] = from_vault;
            }
        }
    }

    /// Collapse the processed vaults in the lowest amount
    /// of vaults needed to hold them. Removes excess vaults.
    pub fn collapse_processed(&mut self) {
        let mut fill_vault_index: usize = 0;
        let mut from_vault_index: usize = self.processed_size().saturating_sub(1);

        while fill_vault_index < from_vault_index {
            if self.processed_vaults[fill_vault_index].size() == self.vault_size {
                fill_vault_index += 1;
            } else if self.processed_vaults[from_vault_index].size() == 0 {
                from_vault_index -= 1;
            } else {
                let fill_size = self.vault_size - self.processed_vaults[fill_vault_index].size();
                // ugly workaround, a more elegant solution might be possible
                let mut from_vault = self.processed_vaults[from_vault_index].clone();
                self.processed_vaults[fill_vault_index].collapse(fill_size, &mut from_vault);
                self.processed_vaults[from_vault_index] = from_vault;
            }
        }
    }

    /// Swap the processing and processed vault. Useful when finishing
    /// an iteration and starting the next.
    /// This function works under the assumption that processing vaults
    /// are empty when called.
    pub fn swap_processing_processed_vaults(&mut self) {
        // Particles are all in front of the list
        self.collapse_processed();

        let mut processed_vault: usize = 0;

        // while there are processing vaults (not empty since we collapsed them before)
        while processed_vault < self.processed_size() {
            core::mem::swap(
                &mut self.processed_vaults[processed_vault],
                &mut self.processing_vaults[processed_vault],
            );
            processed_vault += 1;

            // no more processing vaults?
            if processed_vault == self.processing_size() {
                let mut vault: ParticleVault<T> = ParticleVault {
                    particles: Vec::new(),
                };
                vault.reserve(self.vault_size);
                self.processing_vaults.push(vault);
            }
        }
    }

    /// Add a particle to the specified processing vault.
    pub fn add_processing_particle(
        &mut self,
        particle: MCBaseParticle<T>,
        fill_vault_index: &mut usize,
    ) {
        println!("particle_added");
        while !self.processing_vaults[*fill_vault_index].size() < self.vault_size {
            *fill_vault_index += 1;

            // no more processing vaults?
            if *fill_vault_index == self.processing_size() {
                let mut vault: ParticleVault<T> = ParticleVault {
                    particles: Vec::new(),
                };
                vault.reserve(self.vault_size);
                self.processing_vaults.push(vault);
            }
        }
        self.processing_vaults[*fill_vault_index].push_base_particle(particle);
    }

    /// Add a particle to an extra vault.
    /// Uses an atomic in original code
    pub fn add_extra_particle(&mut self, particle: MCParticle<T>) {
        let vault = self.extra_vault_index / self.vault_size;
        self.extra_vault_index += 1;
        self.extra_vault[vault].push_particle(particle);
    }

    /// Cleans up the extra vaults by moving particles into the processing vaults.
    pub fn clean_extra_vaults(&mut self) {
        let n = self.particles_extra_size();
        if n == 0 {
            return;
        }
        let num_extra = if n % self.vault_size == 0 {
            n / self.vault_size
        } else {
            n / self.vault_size + 1
        };

        let mut extra_idx: usize = 0;
        let mut processing_idx: usize = 0;

        while extra_idx < num_extra {
            if self.extra_vault[extra_idx].size() == 0 {
                // current extra vault empty
                extra_idx += 1;
            } else if processing_idx == self.processing_size() {
                // no more processing vaults
                let mut vault: ParticleVault<T> = ParticleVault {
                    particles: Vec::new(),
                };
                vault.reserve(self.vault_size);
                self.processing_vaults.push(vault);
            } else if self.processing_vaults[processing_idx].size() == self.vault_size {
                // current processing vault full
                processing_idx += 1;
            } else {
                // we move the particle
                let fill_size = self.vault_size - self.processing_vaults[processing_idx].size();
                self.processing_vaults[processing_idx]
                    .collapse(fill_size, &mut self.extra_vault[extra_idx]);
            }
        }
    }
}
