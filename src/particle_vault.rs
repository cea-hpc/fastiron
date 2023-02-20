use num::Float;

use crate::mc::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle};

/// Struture used to group particle in batches.
#[derive(Debug)]
pub struct ParticleVault<T: Float> {
    pub particles: Vec<MCBaseParticle<T>>,
}

impl<T: Float> ParticleVault<T> {
    pub fn append(vault2: &ParticleVault<T>) {
        todo!()
    }

    pub fn collapse(fill_size: usize, vault2: &ParticleVault<T>) {
        todo!()
    }

    pub fn push_particle(particle: MCParticle<T>) {
        todo!()
    }

    pub fn push_base_partcile(particle: MCBaseParticle<T>) {
        todo!()
    }

    pub fn pop_particle(particle: MCParticle<T>) -> bool {
        todo!()
    }

    pub fn pop_base_particle(particle: MCBaseParticle<T>) -> bool {
        todo!()
    }

    pub fn get_particle(particle: MCParticle<T>, index: usize) -> bool {
        todo!()
    }

    pub fn get_base_particle(particle: MCBaseParticle<T>, index: usize) -> bool {
        todo!()
    }

    pub fn put_particle(particle: MCParticle<T>, index: usize) -> bool {
        todo!()
    }

    pub fn invalidate_particle(index: usize) {
        todo!()
    }

    pub fn swap_vaults(vault: &mut ParticleVault<T>) {
        todo!()
    }

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
