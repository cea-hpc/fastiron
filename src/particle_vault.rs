use std::fmt::Debug;

use crate::{
    constants::CustomFloat,
    mc::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle},
};

/// Struture used to group particle in batches.
#[derive(Debug, Clone)]
pub struct ParticleVault<T: CustomFloat> {
    pub particles: Vec<Option<MCBaseParticle<T>>>,
}

impl<T: CustomFloat> Default for ParticleVault<T> {
    fn default() -> Self {
        Self {
            particles: Vec::new(),
        }
    }
}

impl<T: CustomFloat> ParticleVault<T> {
    /// Returns true if the vault is empty, false otherwise.
    pub fn empty(&self) -> bool {
        todo!(); // this is incorrect; empty means full of None
                 //self.particles.is_empty()
    }

    /// Reserve the size for the container of particles.
    pub fn reserve(&mut self, n: usize) {
        // The operation is needed as the reserve method on Vec takes an
        // additional size as argument, not total size.
        // this works if reserve is only called at creation
        self.particles = vec![None; n];
        //self.particles.reserve(n - self.size());
    }

    /// Returns the size of the vault, i.e. the number of VALID particle in the vault.
    pub fn size(&self) -> usize {
        self.particles.iter().filter(|pp| pp.is_some()).count()
    }

    /// Returns the capacity of the vault, i.e. the length of the internal storage vector.
    pub fn capacity(&self) -> usize {
        self.particles.len()
    }

    /// Add all particles of a second vault into this one.
    /// Second vault is left untouched.
    pub fn append(&mut self, vault2: &ParticleVault<T>) {
        self.particles.extend_from_slice(&vault2.particles)
    }

    /// Add all particles of a second vault into this one.
    /// Second vault is left empty. `fill_size` refers to the maximum
    /// possible size for the vault.
    pub fn collapse(&mut self, fill_size: usize, vault2: &mut ParticleVault<T>) {
        if vault2.size() < fill_size {
            // build the new particle list starting with collapsed elements from self
            let mut new: Vec<Option<MCBaseParticle<T>>> = self
                .particles
                .clone()
                .into_iter()
                .filter(|pp| pp.is_some())
                .collect();
            let mut v2_particles: Vec<Option<MCBaseParticle<T>>> = vault2
                .particles
                .clone()
                .into_iter()
                .filter(|pp| pp.is_some())
                .collect();
            new.append(&mut v2_particles);
            while new.len() < self.particles.len() {
                new.push(None);
            }
            vault2.clear();
            self.particles = new;
        } else {
            let mut new: Vec<Option<MCBaseParticle<T>>> = self
                .particles
                .clone()
                .into_iter()
                .filter(|pp| pp.is_some())
                .collect();
            let v2_particles: Vec<Option<MCBaseParticle<T>>> = vault2
                .particles
                .clone()
                .into_iter()
                .filter(|pp| pp.is_some())
                .collect();
            new.extend_from_slice(&v2_particles[..fill_size]);
            self.particles = new;
            vault2.clear();
            (0..v2_particles.len() - fill_size)
                .into_iter()
                .for_each(|ii| {
                    vault2.particles[ii] = v2_particles[fill_size + ii].clone();
                });
        }
    }

    /// Clear all particles from the vault.
    pub fn clear(&mut self) {
        self.particles = vec![None; self.particles.len()];
    }

    /// Put a particle into the vault, casting it into a [MCBaseParticle].
    /// The particle is pushed at the first empty space found, not necessarily after
    /// the last non-empty space.
    /// Fails if the vault has no empty space left (i.e. no None value).
    /// Has an atomic increment in the original code.
    pub fn push_particle(&mut self, particle: MCParticle<T>) {
        let insert_idx = self
            .particles
            .iter()
            .position(|elem| elem.is_none())
            .unwrap();

        self.particles[insert_idx] = Some(MCBaseParticle::new(&particle));
    }

    /// Put a base particle into the vault. The particle is pushed at the first
    /// empty space found, not necessarily after the last non-empty space.
    /// Fails if the vault has no empty space left (i.e. no None value).
    pub fn push_base_particle(&mut self, particle: MCBaseParticle<T>) {
        let insert_idx = self
            .particles
            .iter()
            .position(|elem| elem.is_none())
            .unwrap();
        self.particles[insert_idx] = Some(particle);
    }

    /// Get a particle from the vault.
    /// Note that there is currently no difference made between a None
    /// returned if self.particles is empty and a None returned because
    /// the last particle is invalid. Because of this, this function cannot
    /// be used to detect if the vault is empty.
    pub fn pop_particle(&mut self) -> Option<MCParticle<T>> {
        // find the last valid particle
        if let Some(pp) = self
            .particles
            .iter_mut()
            .filter(|elem| elem.is_some())
            .last()
        {
            // copy it, set it to none and return the copy
            pp.as_ref()?;
            let res = pp.clone();
            *pp = None;
            return Some(MCParticle::new(&res.unwrap()));
        }
        // Currently empty
        None
    }

    /// Get a base particle from the vault.
    /// Note that there is currently no difference made between a None
    /// returned if self.particles is empty and a None returned because
    /// the last particle is invalid. Because of this, this function cannot
    /// be used to detect if the vault is empty.
    pub fn pop_base_particle(&mut self) -> Option<MCBaseParticle<T>> {
        // find the last valid particle
        if let Some(pp) = self
            .particles
            .iter_mut()
            .filter(|elem| elem.is_some())
            .last()
        {
            // copy it, set it to none and return the copy
            pp.as_ref()?;
            let res = pp.clone();
            *pp = None;
            return res;
        }
        // Currently empty
        None
    }

    /// Get the index-corresponding particle from the vault.
    pub fn get_particle(&self, index: usize) -> Option<MCParticle<T>> {
        if let Some(pp) = &self.particles[index] {
            return Some(MCParticle::new(pp));
        }
        None
    }

    /// Get the index-corresponding base particle from the vault.
    pub fn get_base_particle(&self, index: usize) -> Option<MCBaseParticle<T>> {
        self.particles[index].clone()
    }

    /// Put a particle into the vault, at a specific index.
    pub fn put_particle(&mut self, particle: MCParticle<T>, index: usize) {
        if self.particles[index].is_some() {
            println!("WARNING: overwriting particle at index {index}");
        }
        self.particles[index] = Some(MCBaseParticle::new(&particle)); // will panic if out of bounds
    }

    /// Invalidate the particle at the specified index.
    /// Is this really correct? The function is used at the end of each
    /// time iteration, meaning the particle may continue to travel
    /// at next iter
    pub fn invalidate_particle(&mut self, index: usize) {
        self.particles[index] = None; // will panic if out of bounds
    }

    /// Swaps the particle at the specified index with the last one,
    /// delete the last one.
    /// Instead of poping the discarded value, we overwrite it with None
    /// TMP: is the swap really necessary since we don't use pointer to index?
    pub fn erase_swap_particles(&mut self, index: usize) {
        let n = self.particles.len();
        //self.particles[index] = self.particles.pop().unwrap() // does this work?
        if let Some(last_pp_idx) = self.particles.iter().rev().position(|elem| elem.is_some()) {
            self.particles[index] = self.particles[n - 1 - last_pp_idx].clone();
            self.particles[n - 1 - last_pp_idx] = None;
        } else {
            println!("No valid particle to swap with; invalidating the particle");
            self.particles[index] = None;
        }
    }
}

// may be convenient to access particles directly. Either this or get_base_particle
// might be deleted later
impl<T: CustomFloat> core::ops::Index<usize> for ParticleVault<T> {
    type Output = Option<MCBaseParticle<T>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.particles[index]
    }
}

// may be convenient to access particles directly. Either this or get_base_particle
// might be deleted later
impl<T: CustomFloat> core::ops::IndexMut<usize> for ParticleVault<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.particles[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn push() {
        let mut vault = ParticleVault::<f64>::default();
        assert_eq!(vault.size(), 0);
        assert_eq!(vault.capacity(), 0);
        assert!(vault.pop_particle().is_none());

        vault.reserve(1); // vault has the capacity to receive the particle
        assert_eq!(vault.size(), 0);
        assert_eq!(vault.capacity(), 1);
        vault.push_particle(MCParticle::<f64>::default());

        assert!(vault.size() > 0);
        assert!(vault.pop_particle().is_some());
        assert!(vault.pop_particle().is_none());
    }

    #[test]
    #[should_panic]
    fn push_panic() {
        let mut vault = ParticleVault::<f64>::default();
        assert_eq!(vault.size(), 0);
        assert!(vault.pop_particle().is_none());

        //vault.reserve(1);
        // vault is initialized but no space has been reserved
        vault.push_particle(MCParticle::<f64>::default());
    }

    #[test]
    #[should_panic]
    fn basic_panic() {
        let mut vault = ParticleVault::<f64>::default();
        assert_eq!(vault.size(), 0);
        vault.put_particle(Default::default(), 1);
    }
}
