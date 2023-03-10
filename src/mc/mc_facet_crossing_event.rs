use num::{Float, FromPrimitive};

use crate::{montecarlo::MonteCarlo, tallies::MCTallyEvent};

use super::{
    mc_facet_adjacency::MCSubfacetAdjacencyEvent, mc_particle::MCParticle, mct::reflect_particle,
};

/// Computes and transform accordingly a [MCParticle] object crossing a facet.
pub fn facet_crossing_event<T: Float + FromPrimitive>(
    mc_particle: &mut MCParticle<T>,
    mcco: &mut MonteCarlo<T>,
    particle_idx: usize,
    processing_vault_idx: usize,
) -> MCTallyEvent {
    let location = mc_particle.get_location();
    let facet_adjacency = &mcco.domain[location.domain.unwrap()].mesh.cell_connectivity
        [location.cell.unwrap()]
    .facet[location.facet.unwrap()]
    .subfacet;

    match facet_adjacency.event {
        MCSubfacetAdjacencyEvent::TransitOnProcessor => {
            // particle enters an adjacent cell
            mc_particle.domain = facet_adjacency.adjacent.domain.unwrap();
            mc_particle.cell = facet_adjacency.adjacent.cell.unwrap();
            mc_particle.facet = facet_adjacency.adjacent.facet.unwrap();
            mc_particle.last_event = MCTallyEvent::FacetCrossingTransitExit;
        }
        MCSubfacetAdjacencyEvent::BoundaryEscape => {
            // particle escape the system
            mc_particle.last_event = MCTallyEvent::FacetCrossingEscape;
        }
        MCSubfacetAdjacencyEvent::BoundaryReflection => {
            // particle reflect off a system boundary
            mc_particle.last_event = MCTallyEvent::FacetCrossingReflection
        }
        MCSubfacetAdjacencyEvent::TransitOffProcessor => {
            // particle enters an adjacent cell that belongs to
            // a domain managed by another processor.
            mc_particle.domain = facet_adjacency.adjacent.domain.unwrap();
            mc_particle.cell = facet_adjacency.adjacent.cell.unwrap();
            mc_particle.facet = facet_adjacency.adjacent.facet.unwrap();
            mc_particle.last_event = MCTallyEvent::FacetCrossingCommunication;

            // added from cycle tracking; necessary since we dont pass around a pointer
            reflect_particle(mcco, mc_particle);

            let neighbor_rank: usize = mcco.domain[facet_adjacency.current.domain.unwrap()]
                .mesh
                .nbr_rank[facet_adjacency.neighbor_index.unwrap()];
            mcco.particle_vault_container.processing_vaults[processing_vault_idx]
                .put_particle(mc_particle.clone(), particle_idx);

            mcco.particle_vault_container
                .get_send_queue()
                .push(neighbor_rank, particle_idx);
        }
        MCSubfacetAdjacencyEvent::AdjacencyUndefined => panic!(),
    }
    mc_particle.last_event
}
