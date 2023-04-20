//! Event-specific code for particles crossing a cell's facet
//!
//! This module contains code that updates a particle according to the facet
//! it is crossing. See [MCSubfacetAdjacencyEvent] for more information.

use crate::{
    constants::CustomFloat,
    data::{send_queue::SendQueue, tallies::MCTallyEvent},
    geometry::facets::MCSubfacetAdjacencyEvent,
    montecarlo::MonteCarloUnit,
    particles::mc_particle::MCParticle,
};

/// Computes and transform accordingly a [MCParticle] object crossing a facet.
///
/// This functions update a particle's locational data according to one of the
/// four defined adjacency events ([MCSubfacetAdjacencyEvent]). Note that in a
/// sequential or a memory-shared parallelism context, there are no off-processor
/// transit.
pub fn facet_crossing_event<T: CustomFloat>(
    particle: &mut MCParticle<T>,
    mcunit: &MonteCarloUnit<T>,
    send_queue: &mut SendQueue<T>,
) {
    let facet_adjacency = &mcunit.domain[particle.base_particle.domain]
        .mesh
        .cell_connectivity[particle.base_particle.cell]
        .facet[particle.facet]
        .subfacet;

    match facet_adjacency.event {
        MCSubfacetAdjacencyEvent::TransitOnProcessor => {
            // particle enters an adjacent cell
            particle.base_particle.domain = facet_adjacency.adjacent.domain.unwrap();
            particle.base_particle.cell = facet_adjacency.adjacent.cell.unwrap();
            particle.facet = facet_adjacency.adjacent.facet.unwrap();
            particle.base_particle.last_event = MCTallyEvent::FacetCrossingTransitExit;
        }
        MCSubfacetAdjacencyEvent::BoundaryEscape => {
            // particle escape the system
            particle.base_particle.last_event = MCTallyEvent::FacetCrossingEscape;
        }
        MCSubfacetAdjacencyEvent::BoundaryReflection => {
            // particle reflect off a system boundary
            particle.base_particle.last_event = MCTallyEvent::FacetCrossingReflection
        }
        MCSubfacetAdjacencyEvent::TransitOffProcessor => {
            // particle enters an adjacent cell that belongs to
            // a domain managed by another processor.
            particle.base_particle.domain = facet_adjacency.adjacent.domain.unwrap();
            particle.base_particle.cell = facet_adjacency.adjacent.cell.unwrap();
            particle.facet = facet_adjacency.adjacent.facet.unwrap();
            particle.base_particle.last_event = MCTallyEvent::FacetCrossingCommunication;

            let neighbor_rank: usize = mcunit.domain[facet_adjacency.current.domain.unwrap()]
                .mesh
                .nbr_rank[facet_adjacency.neighbor_index.unwrap()];

            send_queue.push(neighbor_rank, particle);
        }
        MCSubfacetAdjacencyEvent::AdjacencyUndefined => panic!(),
    }
}
