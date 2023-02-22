use num::Float;

use crate::mc::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle};

/// Struture used to group particle in batches.
#[derive(Debug)]
pub struct ParticleVault<T: Float> {
    pub particles: Vec<MCBaseParticle<T>>,
}

impl<T: Float> ParticleVault<T> {
    /// Returns true if the vault is empty, false otherwise.
    pub fn empty(&self) -> bool {
        self.particles.is_empty()
    }

    /// Returns the size of the vault.
    pub fn size(&self) -> usize {
        self.particles.len()
    }

    /// Add all particles of a second vault into this one.
    pub fn append(vault2: &ParticleVault<T>) {
        todo!()
    }

    /// Same but consumes the second vault?
    pub fn collapse(fill_size: usize, vault2: ParticleVault<T>) {
        todo!()
    }

    /// Clear all particles from the vault.
    pub fn clear(&mut self) {
        self.particles.clear();
    }

    /// Put a particle into the vault, casting it into a [MCBaseParticle].
    pub fn push_particle(particle: MCParticle<T>) {
        todo!()
    }

    /// Put a base particle into the vault.
    pub fn push_base_partcile(particle: MCBaseParticle<T>) {
        todo!()
    }

    /// Get a particle from the vault. Change to return an option/result asap.
    /// This removes the particle from the vault?
    pub fn pop_particle(particle: MCParticle<T>) -> bool {
        todo!()
    }

    /// Get a base particle from the vault. Change to return an option/result asap.
    /// This removes the particle from the vault?
    pub fn pop_base_particle(particle: MCBaseParticle<T>) -> bool {
        todo!()
    }

    /// Get the index corresponding particle from the vault.
    /// Change to return an option/result asap.
    pub fn get_particle(particle: MCParticle<T>, index: usize) -> bool {
        todo!()
    }

    /// Get the index corresponding base particle from the vault.
    /// Change to return an option/result asap.
    pub fn get_base_particle(&self, index: usize) -> MCBaseParticle<T> {
        todo!()
    }

    /// Put a particle into the vault, at a specific index.
    pub fn put_particle(particle: MCParticle<T>, index: usize) -> bool {
        todo!()
    }

    /// Invalidate the particle at the specified index.
    pub fn invalidate_particle(index: usize) {
        todo!()
    }

    /// Swap vaults with the specified one.
    pub fn swap_vaults(vault: &mut ParticleVault<T>) {
        todo!()
    }

    /// Swaps the particle at the specified index with the last one,
    /// delete the last one. This was done by resizing in C++.
    pub fn erase_swap_particles(index: usize) {
        todo!()
    }
}

impl<T: Float> core::ops::Index<usize> for ParticleVault<T> {
    type Output = MCBaseParticle<T>;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<T: Float> core::ops::IndexMut<usize> for ParticleVault<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}
