//! Code for segment outcome computation, i.e. whichever event the particle
//! undergoes next
//!
//! This module contains the function computing which event a given particle
//! will undergo next. This is done by using the notion of _distance_ to an event
//! and finding the minimum distance between all the possible events.\
//! The particle's position and time to census is also updated, while
//! event-specific modifications are left to event-specific functions. The result
//! is returned using an enum ([`MCSegmentOutcome`])that takes value according to
//! the event.

use std::fmt::Debug;

use atomic::Ordering;
use num::{one, zero};

use crate::{
    constants::CustomFloat,
    data::tallies::MCTallyEvent,
    geometry::facets::{MCNearestFacet, MCSubfacetAdjacencyEvent},
    montecarlo::{MonteCarloData, MonteCarloUnit},
    particles::mc_particle::MCParticle,
    simulation::{macro_cross_section::weighted_macroscopic_cross_section, mct::nearest_facet},
};

/// Enum representing the outcome of the current segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MCSegmentOutcome {
    /// Value for collision event.
    Collision = 0,
    /// Value for facet crossing event.
    FacetCrossing = 1,
    /// Value for census, i.e. no event.
    Census = 2,
}

/// Structure used to handle all distance related data & comparison.
pub struct DistanceHandler<T: CustomFloat> {
    /// Distance to collision.
    pub collision: T,
    /// Distance to facet crossing.
    pub facet_crossing: T,
    /// Distance to census.
    pub census: T,
    /// Current minimum distance.
    pub min_dist: T,
    /// Current outcome.
    pub outcome: MCSegmentOutcome,
}

impl<T: CustomFloat> DistanceHandler<T> {
    /// Update the distance to a given outcome with the provided value.
    /// Also check for a new minimum distance and update the structure
    /// accordingly.
    pub fn update(&mut self, which_outcome: MCSegmentOutcome, dist_outcome: T) {
        match which_outcome {
            MCSegmentOutcome::Collision => self.collision = dist_outcome,
            MCSegmentOutcome::FacetCrossing => self.facet_crossing = dist_outcome,
            MCSegmentOutcome::Census => self.census = dist_outcome,
        }
        if dist_outcome < self.min_dist {
            self.min_dist = dist_outcome;
            self.outcome = which_outcome;
        }
    }

    /// Update the structure to force a collision
    pub fn force_collision(&mut self) {
        self.collision = T::tiny_float();
        self.facet_crossing = T::huge_float();
        self.census = T::huge_float();
        self.min_dist = T::tiny_float();
        self.outcome = MCSegmentOutcome::Collision;
    }
}

impl<T: CustomFloat> Default for DistanceHandler<T> {
    fn default() -> Self {
        let huge_f: T = T::huge_float();
        Self {
            collision: huge_f,
            facet_crossing: huge_f,
            census: huge_f,
            min_dist: huge_f,
            outcome: MCSegmentOutcome::Collision,
        }
    }
}

impl<T: CustomFloat> MCParticle<T> {
    /// Computes the outcome of the current segment for a given particle.
    ///
    /// Three outcomes are possible for a given particle: census, facet crossing or
    /// collision. The distances are computed as follows:
    ///
    /// - Census: The distance is simply equal to the speed of the particle multiplied
    ///   by the time left until census.
    /// - Facet crossing: The distance is computed in a similar way by the function
    ///   [`nearest_facet()`].
    /// - Collision: The distance is computed using probabilities.
    pub fn outcome(&mut self, mcdata: &MonteCarloData<T>, mcunit: &MonteCarloUnit<T>) {
        //==========
        // Prep work
        self.num_segments += one();
        // update scalar flux tally
        /*mcunit.tallies.scalar_flux_domain[(self.cell, self.energy_group)]
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                        Some(x + self.segment_path_length * self.weight)
                    })
                    .unwrap();
        */
        // initialize distances and constants
        let small_f: T = T::small_float();
        let mut distance_handler = DistanceHandler::default();

        let particle_speed = self.get_speed();

        let mut force_collision = false;
        if self.num_mean_free_paths < zero() {
            force_collision = true;
            self.num_mean_free_paths = small_f;
        }

        // get cross section
        // lazily computed
        // This ordering should make it so that we dont compute a XS multiple times?
        let pcxs = mcunit.xs_cache[(self.cell, self.energy_group)].load(Ordering::Acquire);
        let macroscopic_total_xsection = if pcxs > zero() {
            // use precomputed value
            pcxs
        } else {
            // compute & cache value
            let tmp = weighted_macroscopic_cross_section(
                mcdata,
                self.mat_gid,
                self.cell_nb_density,
                self.energy_group,
            );
            mcunit.xs_cache[(self.cell, self.energy_group)].store(tmp, Ordering::Release);
            tmp
        };

        // prepare particle
        self.total_cross_section = macroscopic_total_xsection;
        if macroscopic_total_xsection == zero() {
            self.mean_free_path = T::huge_float();
        } else {
            self.mean_free_path = one::<T>() / macroscopic_total_xsection;
        }

        // if zero
        if self.num_mean_free_paths == zero() {
            self.sample_num_mfp();
        }

        //===========================
        // Compute distances to event

        // collision
        distance_handler.update(
            MCSegmentOutcome::Collision,
            self.num_mean_free_paths * self.mean_free_path,
        );
        // census
        distance_handler.update(
            MCSegmentOutcome::Census,
            particle_speed * self.time_to_census,
        );
        // nearest facet
        let nearest_facet: MCNearestFacet<T> = nearest_facet(self, &mcunit.domain.mesh);
        distance_handler.update(
            MCSegmentOutcome::FacetCrossing,
            nearest_facet.distance_to_facet,
        );

        // force a collision if needed
        if force_collision {
            distance_handler.force_collision();
        }

        //============================================
        // Pick the outcome & update particle, tallies

        let segment_outcome = distance_handler.outcome;
        assert!(distance_handler.min_dist >= zero());

        // general update
        self.segment_path_length = distance_handler.min_dist;
        self.num_mean_free_paths -= self.segment_path_length / self.mean_free_path;

        // outcome-specific updates
        match segment_outcome {
            MCSegmentOutcome::Collision => {
                self.num_mean_free_paths = zero();
                self.last_event = MCTallyEvent::Collision;
            }
            MCSegmentOutcome::FacetCrossing => {
                self.facet = nearest_facet.facet;
                let facet_adjacency =
                    &mcunit.domain.mesh.cell_connectivity[self.cell].facet[self.facet].subfacet;
                match facet_adjacency.event {
                    MCSubfacetAdjacencyEvent::TransitOnProcessor => {
                        // particle enters an adjacent cell
                        self.domain = facet_adjacency.adjacent.domain.unwrap();
                        self.cell = facet_adjacency.adjacent.cell.unwrap();
                        self.facet = facet_adjacency.adjacent.facet.unwrap();
                        self.mat_gid = mcunit.domain.cell_state[self.cell].material;
                        self.cell_nb_density =
                            mcunit.domain.cell_state[self.cell].cell_number_density;
                        self.last_event = MCTallyEvent::FacetCrossingTransitExit;
                    }
                    MCSubfacetAdjacencyEvent::BoundaryEscape => {
                        // particle escape the system
                        self.last_event = MCTallyEvent::FacetCrossingEscape;
                    }
                    MCSubfacetAdjacencyEvent::BoundaryReflection => {
                        // particle reflect off a system boundary
                        self.last_event = MCTallyEvent::FacetCrossingReflection;
                        self.facet_normal =
                            mcunit.domain.mesh.cell_geometry[self.cell][self.facet].get_normal();
                    }
                    MCSubfacetAdjacencyEvent::TransitOffProcessor => {
                        // particle enters an adjacent cell that belongs to
                        // a domain managed by another processor.
                        self.domain = facet_adjacency.adjacent.domain.unwrap();
                        self.cell = facet_adjacency.adjacent.cell.unwrap();
                        self.facet = facet_adjacency.adjacent.facet.unwrap();
                        self.mat_gid = mcunit.domain.cell_state[self.cell].material;
                        self.cell_nb_density =
                            mcunit.domain.cell_state[self.cell].cell_number_density;
                        self.last_event = MCTallyEvent::FacetCrossingCommunication;
                    }
                    MCSubfacetAdjacencyEvent::AdjacencyUndefined => panic!(),
                }
            }
            MCSegmentOutcome::Census => {
                self.time_to_census = zero::<T>().min(self.time_to_census);
                self.last_event = MCTallyEvent::Census;
            }
        }

        // skip tallies & early return if the path length is 0
        if self.segment_path_length == zero() {
            return;
        }

        // move particle to the end of the segment
        self.move_particle_along_segment();

        // decrement time to census & increment age
        let segment_path_time = self.segment_path_length / particle_speed;
        self.time_to_census -= segment_path_time;
        self.age += segment_path_time;
        if self.time_to_census < zero() {
            self.time_to_census = zero();
        }
    }
}
