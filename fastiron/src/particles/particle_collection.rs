//! Data structure used to hold particles
//!
//! This module contains code used as an abstraction of particle vectors. This
//! allows for custom iterator implementation.

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::constants::CustomFloat;

use super::mc_particle::MCParticle;

pub struct ParticleCollection<T: CustomFloat> {
    data: Vec<MCParticle<T>>,
}

pub struct ParParIter<'a, T: CustomFloat> {
    data_slice: &'a [MCParticle<T>],
}

impl<'a, T: CustomFloat> ParallelIterator for ParParIter<'a, T> {
    type Item = &'a MCParticle<T>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        todo!()
    }

    fn opt_len(&self) -> Option<usize> {
        todo!()
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
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}

impl<'a, T: CustomFloat> IntoParallelIterator for &'a ParticleCollection<T> {
    type Iter = ParParIter<'a, T>;
    type Item = &'a MCParticle<T>;

    fn into_par_iter(self) -> Self::Iter {
        todo!()
    }
}

impl<'a, T: CustomFloat> IntoIterator for &'a ParticleCollection<T> {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = &'a MCParticle<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
