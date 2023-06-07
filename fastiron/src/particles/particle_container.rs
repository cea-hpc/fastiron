//! Data structure used to hold particles
//!
//! This module contains code used for the main structure holding particles.

use std::sync::{atomic::Ordering, Arc, Mutex};

use rayon::prelude::*;

use crate::{
    constants::CustomFloat,
    data::tallies::Balance,
    montecarlo::{MonteCarloData, MonteCarloUnit},
    simulation::cycle_tracking::{cycle_tracking_guts, par_cycle_tracking_guts},
    utils::mc_processor_info::ExecPolicy,
};

use super::{mc_particle::Species, particle_collection::ParticleCollection};

#[derive(Debug, Clone)]
/// Structure used as a container for all particles.
///
/// The [Clone] implementation should not be used except at the beginning of the program.
pub struct ParticleContainer<T: CustomFloat> {
    /// Container for particles that have yet to be processed.
    pub processing_particles: ParticleCollection<T>,
    /// Container for already processed particles.
    pub processed_particles: ParticleCollection<T>,
    /// Container for extra particles. This is used for fission-induced
    /// particles and incoming off-processor particles.
    pub extra_particles: ParticleCollection<T>,
}

impl<T: CustomFloat> ParticleContainer<T> {
    /// Constructor. The appropriate capacity is computed beforehand.
    pub fn new(regular_capacity: usize, extra_capacity: usize) -> Self {
        Self {
            processing_particles: ParticleCollection::with_capacity(regular_capacity),
            processed_particles: ParticleCollection::with_capacity(regular_capacity),
            extra_particles: ParticleCollection::with_capacity(extra_capacity),
        }
    }

    /// Swap the processing and processed particle lists. This function is used in-between
    /// iterations.
    pub fn swap_processing_processed(&mut self) {
        self.processing_particles
            .append(&mut self.processed_particles);
        //core::mem::swap(
        //    &mut self.processing_particles,
        //    &mut self.processed_particles,
        //);
    }

    /// Randomly delete particles to reach the desired number of particles for
    /// simulation. Low weight particles are, then, randomly deleted.
    pub fn regulate_population(
        &mut self,
        split_rr_factor: T,
        relative_weight_cutoff: T,
        source_particle_weight: T,
        balance: &Balance,
    ) {
        let old_len = self.processing_particles.len();
        self.processing_particles.retain_mut(|pp| {
            let survive_once = pp.over_populated_rr(split_rr_factor);
            let survive_twice = pp.low_weight_rr(relative_weight_cutoff, source_particle_weight);
            survive_once & survive_twice
        });
        balance.rr.fetch_add(
            (old_len - self.processing_particles.len()) as u64,
            Ordering::Relaxed,
        );
    }

    /// Split particles to reach the desired number of particles for
    /// simulation. Low weight particles are, then, randomly deleted.
    pub fn split_population(
        &mut self,
        split_rr_factor: T,
        relative_weight_cutoff: T,
        source_particle_weight: T,
        balance: &Balance,
    ) {
        let mut old_len = self.processing_particles.len();
        (&mut self.processing_particles).into_iter().for_each(|pp| {
            self.extra_particles
                .extend(pp.under_populated_split(split_rr_factor));
        });
        self.clean_extra_vaults();
        balance.split.fetch_add(
            (self.processing_particles.len() - old_len) as u64,
            Ordering::Relaxed,
        );
        old_len = self.processing_particles.len();
        self.processing_particles
            .retain_mut(|pp| pp.low_weight_rr(relative_weight_cutoff, source_particle_weight));
        balance.rr.fetch_add(
            (old_len - self.processing_particles.len()) as u64,
            Ordering::Relaxed,
        );
    }

    /// Track particles and transfer them to the processed storage when done.
    pub fn process_particles(&mut self, mcdata: &MonteCarloData<T>, mcunit: &MonteCarloUnit<T>) {
        self.sort_processing();
        match mcdata.exec_info.exec_policy {
            // Process unit sequentially
            ExecPolicy::Sequential | ExecPolicy::Distributed => {
                (&mut self.processing_particles)
                    .into_iter()
                    .for_each(|particle| {
                        cycle_tracking_guts(mcdata, mcunit, particle, &mut self.extra_particles)
                    });
            }
            // Process unit in parallel
            ExecPolicy::Rayon | ExecPolicy::Hybrid => {
                let extra_capacity = self.extra_particles.capacity() / 4;
                let extra = Arc::new(Mutex::new(&mut self.extra_particles));
                // choose chunk size to get one chunk per thread
                let chunk_size: usize =
                    (self.processing_particles.len() / mcdata.exec_info.n_rayon_threads) + 1;

                self.processing_particles
                    .par_iter_mut()
                    .chunks(chunk_size)
                    .map(|mut particles| {
                        // Strategy used to reduce ressource (memory) contention
                        // 1. Give each chunks (==thread with our chunk_size value) its own balance
                        //    This removes the need for atomics type. The tradeoff: folding results
                        //    of the iterators
                        // 2. Use a local extra collection that is later used to extend the global
                        //    container. This reduces the total number of lock (and prolly lock time)
                        let mut local_balance: Balance = Balance::default();
                        let mut local_extra: ParticleCollection<T> =
                            ParticleCollection::with_capacity(extra_capacity);
                        particles.iter_mut().for_each(|particle| {
                            par_cycle_tracking_guts(mcdata, mcunit, particle, &mut local_extra)
                        });
                        extra.lock().unwrap().append(&mut local_extra);
                        local_balance
                    })
                    .fold_with(Balance::default(), |a, b| a.add(&b));
            }
        }
        self.processing_particles
            .retain(|particle| particle.species != Species::Unknown);
        self.processed_particles
            .append(&mut self.processing_particles);
    }

    /// Sort the processing particles according to where they belong in the mesh.
    pub fn sort_processing(&mut self) {
        self.processing_particles
            .sort_by(|a, b| match a.cell.cmp(&b.cell) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => a.energy_group.cmp(&b.energy_group),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            });
    }

    /// Processes the particles stored in the send queue.
    /// - In a shared memory context, this is just a transfer from the send queue
    ///   to the extra storage
    /// - In a message-passing context, this would include sending and receiving
    ///   particles
    /*
        pub fn process_sq(&mut self) {
            self.extra_particles.extend(
                self.send_queue
                    .data
                    .iter()
                    .map(|sq_tuple| sq_tuple.particle.clone()),
            );
            self.send_queue.clear();
            // Here we would add the receiver part
            // while rx.try_recv().is_ok() {...}
        }
    */

    /// Adds back to the processing storage the extra particles.
    pub fn clean_extra_vaults(&mut self) {
        self.processing_particles.append(&mut self.extra_particles);
    }

    /// Checks if there are no more particles to process, i.e:
    /// - extra storage is empty
    /// - processing storage is empty
    /// - send queue is empty
    pub fn test_done_new(&self) -> bool {
        self.extra_particles.is_empty() & self.processing_particles.is_empty()
    }
}
