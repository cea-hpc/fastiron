use num::{Float, FromPrimitive};

use crate::{montecarlo::MonteCarlo, particle_vault::ParticleVault, tallies::MCTallyEvent};

use super::{
    mc_facet_adjacency::MCSubfacetAdjacencyEvent, mc_particle::MCParticle, mct::reflect_particle,
};

/// Computes and transform accordingly a [MCParticle] object crossing a facet.
pub fn event<T: Float + FromPrimitive>(
    mc_particle: &mut MCParticle<T>,
    mcco: &mut MonteCarlo<T>,
    particle_idx: usize,
    processing_vault: &mut ParticleVault<T>,
) -> MCTallyEvent {
    let location = mc_particle.get_location();
    let facet_adjacency = &mcco.domain[location.domain].mesh.cell_connectivity[location.cell].facet
        [location.facet]
        .subfacet;

    match facet_adjacency.event {
        MCSubfacetAdjacencyEvent::TransitOnProcessor => {
            // particle enters an adjacent cell
            mc_particle.domain = facet_adjacency.adjacent.domain;
            mc_particle.cell = facet_adjacency.adjacent.cell;
            mc_particle.facet = facet_adjacency.adjacent.facet;
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
            mc_particle.domain = facet_adjacency.adjacent.domain;
            mc_particle.cell = facet_adjacency.adjacent.cell;
            mc_particle.facet = facet_adjacency.adjacent.facet;
            mc_particle.last_event = MCTallyEvent::FacetCrossingCommunication;

            reflect_particle(mcco, mc_particle); // added from cycle tracking

            let neighbor_rank: usize = mcco.domain[facet_adjacency.current.domain].mesh.nbr_rank
                [facet_adjacency.neighbor_index];
            processing_vault.put_particle(mc_particle.clone(), particle_idx);

            mcco.particle_vault_container
                .get_send_queue()
                .push(neighbor_rank, particle_idx);
        }
        MCSubfacetAdjacencyEvent::AdjacencyUndefined => panic!(),
    }

    mc_particle.last_event
}
