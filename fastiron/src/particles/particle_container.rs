//! Data structure used to hold particles
//!
//! This module contains code used for the main structure holding particles.

use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use crate::{
    constants::CustomFloat,
    data::tallies::{Balance, TalliedEvent},
    montecarlo::{MonteCarloData, MonteCarloUnit},
    simulation::cycle_tracking::cycle_tracking_guts,
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
        balance: &mut Balance,
    ) {
        let old_len = self.processing_particles.len();
        self.processing_particles.retain_mut(|pp| {
            let survive_once = pp.over_populated_rr(split_rr_factor);
            let survive_twice = pp.low_weight_rr(relative_weight_cutoff, source_particle_weight);
            survive_once & survive_twice
        });
        balance[TalliedEvent::OverRr] = (old_len - self.processing_particles.len()) as u64;
    }

    /// Split particles to reach the desired number of particles for
    /// simulation. Low weight particles are, then, randomly deleted.
    pub fn split_population(
        &mut self,
        split_rr_factor: T,
        relative_weight_cutoff: T,
        source_particle_weight: T,
        balance: &mut Balance,
    ) {
        let mut old_len = self.processing_particles.len();
        (&mut self.processing_particles).into_iter().for_each(|pp| {
            self.extra_particles
                .extend(pp.under_populated_split(split_rr_factor));
        });
        self.clean_extra_vaults();
        balance[TalliedEvent::Split] = (self.processing_particles.len() - old_len) as u64;
        old_len = self.processing_particles.len();
        self.processing_particles
            .retain_mut(|pp| pp.low_weight_rr(relative_weight_cutoff, source_particle_weight));
        balance[TalliedEvent::WeightRr] = (old_len - self.processing_particles.len()) as u64;
    }

    /// Track particles and transfer them to the processed storage when done.
    pub fn process_particles(
        &mut self,
        mcdata: &MonteCarloData<T>,
        mcunit: &mut MonteCarloUnit<T>,
    ) {
        let exinf = &mcdata.exec_info;
        match exinf.exec_policy {
            // Process unit sequentially
            ExecPolicy::Sequential | ExecPolicy::Distributed => {
                let mut tmp = Balance::default();
                (&mut self.processing_particles)
                    .into_iter()
                    .for_each(|particle| {
                        cycle_tracking_guts(
                            mcdata,
                            mcunit,
                            particle,
                            &mut tmp,
                            &mut self.extra_particles,
                        )
                    });
                mcunit.tallies.balance_cycle.add_to_self(&tmp);
            }
            // Process unit in parallel
            ExecPolicy::Rayon | ExecPolicy::Hybrid => {
                let extra = Arc::new(Mutex::new(&mut self.extra_particles));
                // choose chunk size to get one chunk per thread
                let chunk_size: usize = match exinf.chunk_size {
                    0 => (self.processing_particles.len() / exinf.n_rayon_threads) + 1,
                    _ => exinf.chunk_size,
                };

                let res: Balance = self
                    .processing_particles
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
                        // chunk_size * 5 is enough capacity to handle all particles undergoing
                        // fission & splitting into the max possible nb of particles.
                        let mut local_extra: ParticleCollection<T> =
                            ParticleCollection::with_capacity(chunk_size * 5);
                        particles.iter_mut().for_each(|particle| {
                            cycle_tracking_guts(
                                mcdata,
                                mcunit,
                                particle,
                                &mut local_balance,
                                &mut local_extra,
                            )
                        });
                        extra.lock().unwrap().append(&mut local_extra);
                        local_balance
                    })
                    .fold_with(Balance::default(), |a, b| a + b)
                    .sum::<Balance>();
                // It should be safe to simply add this to the one in mcunit
                assert_eq!(res[TalliedEvent::Start], 0);
                assert_eq!(res[TalliedEvent::End], 0);
                assert_eq!(res[TalliedEvent::Source], 0);
                mcunit.tallies.balance_cycle.add_to_self(&res);
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
    pub fn is_done_processing(&self) -> bool {
        self.extra_particles.is_empty() & self.processing_particles.is_empty()
    }
}
