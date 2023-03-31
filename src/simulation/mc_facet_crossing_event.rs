use crate::{
    constants::CustomFloat, data::tallies::MCTallyEvent,
    geometry::mc_facet_adjacency::MCSubfacetAdjacencyEvent, montecarlo::MonteCarlo,
    particles::mc_particle::MCParticle,
};

/// Computes and transform accordingly a [MCParticle] object crossing a facet.
pub fn facet_crossing_event<T: CustomFloat>(
    particle: &mut MCParticle<T>,
    mcco: &mut MonteCarlo<T>,
) -> MCTallyEvent {
    let location = particle.get_location();
    let facet_adjacency = &mcco.domain[location.domain.unwrap()].mesh.cell_connectivity
        [location.cell.unwrap()]
    .facet[location.facet.unwrap()]
    .subfacet;

    match facet_adjacency.event {
        MCSubfacetAdjacencyEvent::TransitOnProcessor => {
            // particle enters an adjacent cell
            particle.domain = facet_adjacency.adjacent.domain.unwrap();
            particle.cell = facet_adjacency.adjacent.cell.unwrap();
            particle.facet = facet_adjacency.adjacent.facet.unwrap();
            particle.last_event = MCTallyEvent::FacetCrossingTransitExit;
        }
        MCSubfacetAdjacencyEvent::BoundaryEscape => {
            // particle escape the system
            particle.last_event = MCTallyEvent::FacetCrossingEscape;
        }
        MCSubfacetAdjacencyEvent::BoundaryReflection => {
            // particle reflect off a system boundary
            particle.last_event = MCTallyEvent::FacetCrossingReflection
        }
        MCSubfacetAdjacencyEvent::TransitOffProcessor => {
            // particle enters an adjacent cell that belongs to
            // a domain managed by another processor.
            particle.domain = facet_adjacency.adjacent.domain.unwrap();
            particle.cell = facet_adjacency.adjacent.cell.unwrap();
            particle.facet = facet_adjacency.adjacent.facet.unwrap();
            particle.last_event = MCTallyEvent::FacetCrossingCommunication;

            let neighbor_rank: usize = mcco.domain[facet_adjacency.current.domain.unwrap()]
                .mesh
                .nbr_rank[facet_adjacency.neighbor_index.unwrap()];

            mcco.particle_vault_container
                .send_queue
                .push(neighbor_rank, particle);
        }
        MCSubfacetAdjacencyEvent::AdjacencyUndefined => panic!(),
    }
    particle.last_event
}
