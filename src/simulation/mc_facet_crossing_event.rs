//! Event-specific code for particles crossing a cell's facet
//!
//! This module contains code that updates a particle according to the facet
//! it is crossing. See [MCSubfacetAdjacencyEvent] for more information.

use crate::{
    constants::CustomFloat,
    data::tallies::MCTallyEvent,
    geometry::facets::{MCSubfacetAdjacencyEvent, SubfacetAdjacency},
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
    facet_adjacency: &SubfacetAdjacency,
) {
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
        }
        MCSubfacetAdjacencyEvent::AdjacencyUndefined => panic!(),
    }
}
