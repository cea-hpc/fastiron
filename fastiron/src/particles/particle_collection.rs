//! Data structure used to hold particles
//!
//! This module contains code used as an abstraction of particle vectors. This
//! allows for custom iterator implementation.

use rayon::{
    iter::plumbing::{bridge, Producer},
    prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator},
};

use crate::constants::CustomFloat;

use super::mc_particle::MCParticle;

//==========================
// Base structures & methods
//==========================

/// Custom data structure used to implement [rayon]'s iterator.
#[derive(Debug, Clone)]
pub struct ParticleCollection<T: CustomFloat> {
    /// Vector holding all the particles of the collection.
    data: Vec<MCParticle<T>>,
}

impl<T: CustomFloat> ParticleCollection<T> {
    /// Propagating method.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Propagating method.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Propagating method.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Propagating method.
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Propagating method.
    pub fn retain<F: Fn(&MCParticle<T>) -> bool>(&mut self, f: F) {
        self.data.retain(f);
    }

    /// Propagating method.
    pub fn retain_mut<F: FnMut(&mut MCParticle<T>) -> bool>(&mut self, f: F) {
        self.data.retain_mut(f);
    }

    /// Propagating method.
    pub fn append(&mut self, other: &mut Self) {
        self.data.append(&mut other.data)
    }

    /// Propagating method.
    pub fn extend<I: IntoIterator<Item = MCParticle<T>>>(&mut self, iter: I) {
        self.data.extend(iter);
    }

    /// Propagating method.
    pub fn sort_by<F: FnMut(&MCParticle<T>, &MCParticle<T>) -> std::cmp::Ordering>(
        &mut self,
        compare: F,
    ) {
        self.data.sort_by(compare);
    }
}

/// Custom immutable iterator structure.
pub struct ParParIter<'a, T: CustomFloat> {
    data_slice: &'a [MCParticle<T>],
}

/// Custom mutable iterator structure.
pub struct ParParIterMut<'a, T: CustomFloat> {
    data_slice: &'a mut [MCParticle<T>],
}

/// Custom immutable producer for the iterator to use.
pub struct ParticleProducer<'a, T: CustomFloat> {
    data_slice: &'a [MCParticle<T>],
}

/// Custom mutable producer for the iterator to use.
pub struct ParticleProducerMut<'a, T: CustomFloat> {
    data_slice: &'a mut [MCParticle<T>],
}

//==========
// // traits
//==========

// immutable

impl<'a, T: CustomFloat> ParallelIterator for ParParIter<'a, T> {
    type Item = &'a MCParticle<T>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'a, T: CustomFloat> IndexedParallelIterator for ParParIter<'a, T> {
    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(
        self,
        callback: CB,
    ) -> CB::Output {
        let producer = ParticleProducer::from(self);
        callback.callback(producer)
    }

    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.data_slice.len()
    }
}

impl<'a, T: CustomFloat> IntoParallelIterator for &'a ParticleCollection<T> {
    type Iter = ParParIter<'a, T>;
    type Item = &'a MCParticle<T>;

    fn into_par_iter(self) -> Self::Iter {
        ParParIter {
            data_slice: &self.data,
        }
    }
}

// mutable

impl<'a, T: CustomFloat> ParallelIterator for ParParIterMut<'a, T> {
    type Item = &'a mut MCParticle<T>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'a, T: CustomFloat> IndexedParallelIterator for ParParIterMut<'a, T> {
    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(
        self,
        callback: CB,
    ) -> CB::Output {
        let producer = ParticleProducerMut::from(self);
        callback.callback(producer)
    }

    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.data_slice.len()
    }
}

impl<'a, T: CustomFloat> IntoParallelIterator for &'a mut ParticleCollection<T> {
    type Iter = ParParIterMut<'a, T>;
    type Item = &'a mut MCParticle<T>;

    fn into_par_iter(self) -> Self::Iter {
        ParParIterMut {
            data_slice: &mut self.data,
        }
    }
}

//================
// Producer traits
//================

// immutable

impl<'a, T: CustomFloat> Producer for ParticleProducer<'a, T> {
    type IntoIter = std::slice::Iter<'a, MCParticle<T>>;
    type Item = &'a MCParticle<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_slice.iter()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.data_slice.split_at(index);
        (
            ParticleProducer { data_slice: left },
            ParticleProducer { data_slice: right },
        )
    }
}

impl<'a, T: CustomFloat> From<ParParIter<'a, T>> for ParticleProducer<'a, T> {
    fn from(iter: ParParIter<'a, T>) -> Self {
        Self {
            data_slice: iter.data_slice,
        }
    }
}

// mutable

impl<'a, T: CustomFloat> Producer for ParticleProducerMut<'a, T> {
    type IntoIter = std::slice::IterMut<'a, MCParticle<T>>;
    type Item = &'a mut MCParticle<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_slice.iter_mut()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.data_slice.split_at_mut(index);
        (
            ParticleProducerMut { data_slice: left },
            ParticleProducerMut { data_slice: right },
        )
    }
}

impl<'a, T: CustomFloat> From<ParParIterMut<'a, T>> for ParticleProducerMut<'a, T> {
    fn from(iter: ParParIterMut<'a, T>) -> Self {
        Self {
            data_slice: iter.data_slice,
        }
    }
}

//==========
// / traits
//==========

impl<'a, T: CustomFloat> IntoIterator for &'a ParticleCollection<T> {
    type IntoIter = std::slice::Iter<'a, MCParticle<T>>;
    type Item = &'a MCParticle<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T: CustomFloat> IntoIterator for &'a mut ParticleCollection<T> {
    type IntoIter = std::slice::IterMut<'a, MCParticle<T>>;
    type Item = &'a mut MCParticle<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
