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

//================
// Base structures
//================

pub struct ParticleCollection<T: CustomFloat> {
    data: Vec<MCParticle<T>>,
}

pub struct ParParIter<'a, T: CustomFloat> {
    data_slice: &'a [MCParticle<T>],
}

pub struct ParticleProducer<'a, T: CustomFloat> {
    data_slice: &'a [MCParticle<T>],
}

//==========
// // traits
//==========

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
        todo!()
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

//================
// Producer traits
//================

impl<'a, T: CustomFloat> Producer for ParticleProducer<'a, T> {
    type IntoIter = std::slice::Iter<'a, MCParticle<T>>;
    type Item = &'a MCParticle<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        todo!()
    }
}

//==========
// / traits
//==========

impl<T: CustomFloat> IntoIterator for ParticleCollection<T> {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = MCParticle<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

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
