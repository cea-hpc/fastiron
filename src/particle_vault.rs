use num::Float;

use crate::mc::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle};

/// Struture used to group particle in batches.
#[derive(Debug, Clone)]
pub struct ParticleVault<T: Float> {
    pub particles: Vec<MCBaseParticle<T>>,
}

impl<T: Float> ParticleVault<T> {
    /// Returns true if the vault is empty, false otherwise.
    pub fn empty(&self) -> bool {
        self.particles.is_empty()
    }

    /// Reserve the size for the container of particles.
    pub fn reserve(&mut self, n: usize) {
        // The operation is needed as the reserve method on Vec takes an
        // additional size as argument, not total size.
        self.particles.reserve(n - self.size());
    }

    /// Returns the size of the vault.
    pub fn size(&self) -> usize {
        self.particles.len()
    }

    /// Add all particles of a second vault into this one.
    /// Second vault is left untouched.
    pub fn append(&mut self, vault2: &ParticleVault<T>) {
        self.particles.extend_from_slice(&vault2.particles)
    }

    /// Add all particles of a second vault into this one.
    /// Second vault is left empty.
    /// NEED TO TEST IF THE BEHAVIOR DIFFERES FROM THE ORIGINAL FUNCTION
    pub fn collapse(&mut self, fill_size: usize, vault2: &mut ParticleVault<T>) {
        if vault2.size() < fill_size {
            self.particles.append(&mut vault2.particles);
        } else {
            self.particles
                .extend_from_slice(&vault2.particles[..fill_size]);
            vault2.particles = Vec::from(&vault2.particles[fill_size..]);
        }
    }

    /// Clear all particles from the vault.
    pub fn clear(&mut self) {
        self.particles.clear();
    }

    /// Put a particle into the vault, casting it into a [MCBaseParticle].
    /// Has an atomic increment in the original code.
    pub fn push_particle(&mut self, particle: MCParticle<T>) {
        self.particles.push(MCBaseParticle::new(&particle));
    }

    /// Put a base particle into the vault.
    pub fn push_base_particle(&mut self, particle: MCBaseParticle<T>) {
        self.particles.push(particle);
    }

    /// Get a particle from the vault. Change to return an option/result asap.
    pub fn pop_particle(&mut self) -> Option<MCParticle<T>> {
        if let Some(pp) = self.particles.pop() {
            return Some(MCParticle::new(&pp));
        }
        None
    }

    /// Get a base particle from the vault. Change to return an option/result asap.
    pub fn pop_base_particle(&mut self) -> Option<MCBaseParticle<T>> {
        self.particles.pop()
    }

    /// Get the index-corresponding particle from the vault.
    pub fn get_particle(&self, index: usize) -> Option<MCParticle<T>> {
        if let Some(pp) = self.particles.get(index) {
            return Some(MCParticle::new(pp));
        }
        None
    }

    /// Get the index-corresponding base particle from the vault.
    pub fn get_base_particle(&self, index: usize) -> Option<MCBaseParticle<T>> {
        self.particles.get(index).cloned()
    }

    /// Put a particle into the vault, at a specific index.
    pub fn put_particle(&mut self, particle: MCParticle<T>, index: usize) {
        self.particles[index] = MCBaseParticle::new(&particle); // will panic if out of bounds
    }

    /// Invalidate the particle at the specified index.
    pub fn invalidate_particle(&mut self, index: usize) {
        self.particles[index].species = -1; // will panic if out of bounds
    }

    /*
    /// Swap vaults with the specified one. Undefined in original code?
    pub fn swap_vaults(vault: &mut ParticleVault<T>) {
        todo!()
    }
    */

    /// Swaps the particle at the specified index with the last one,
    /// delete the last one.
    pub fn erase_swap_particles(&mut self, index: usize) {
        self.particles[index] = self.particles.pop().unwrap() // does this work?
    }
}

// may be convenient to access particles directly. Either this or get_base_particle
// might be deleted later
impl<T: Float> core::ops::Index<usize> for ParticleVault<T> {
    type Output = MCBaseParticle<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.particles[index]
    }
}

// may be convenient to access particles directly. Either this or get_base_particle
// might be deleted later
impl<T: Float> core::ops::IndexMut<usize> for ParticleVault<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.particles[index]
    }
}
