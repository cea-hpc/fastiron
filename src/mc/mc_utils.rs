use num::{zero, FromPrimitive};

use crate::{
    constants::{
        physical::{LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY, TINY_FLOAT},
        CustomFloat,
    },
    montecarlo::MonteCarlo,
    particle_vault::ParticleVault,
};

use super::{
    mc_base_particle::MCBaseParticle,
    mc_particle::MCParticle,
    mc_rng_state::{rng_sample, spawn_rn_seed},
    mct::generate_coordinate_3dg,
};

/// Copies a single particle from the particle-vault data and returns it.
pub fn load_particle<T: CustomFloat>(
    particle_vault: &ParticleVault<T>,
    particle_idx: usize,
    ts: f64,
) -> Option<MCParticle<T>> {
    //println!("{:#?}", particle_vault.particles);
    // can probably use a map here
    if let Some(mut particle) = particle_vault.get_base_particle(particle_idx) {
        // update time to census
        let tiny_f: T = FromPrimitive::from_f64(TINY_FLOAT).unwrap();
        let time_step: T = FromPrimitive::from_f64(ts).unwrap();
        //println!("loaded particle #{particle_idx}");
        if particle.time_to_census <= tiny_f {
            particle.time_to_census += time_step;
        }
        // set age
        if particle.age < zero() {
            particle.age = zero();
        }

        return Some(MCParticle::new(&particle));
    }
    None
}

/// Simulates the sources according to the problem's parameters.
pub fn source_now<T: CustomFloat>(mcco: &mut MonteCarlo<T>) {
    println!("---source_now");
    let time_step = FromPrimitive::from_f64(mcco.time_info.time_step).unwrap();

    let mut source_rate: Vec<T> = vec![zero(); mcco.material_database.mat.len()];
    (0..mcco.material_database.mat.len())
        .into_iter()
        .for_each(|mat_idx| {
            let name = &mcco.material_database.mat[mat_idx].name;
            let sr = mcco.params.material_params[name].source_rate;
            source_rate[mat_idx] = FromPrimitive::from_f64(sr).unwrap();
        });

    let mut total_weight_particles: T = zero();
    mcco.domain.iter().for_each(|dom| {
        dom.cell_state.iter().for_each(|cell| {
            let cell_weight_particles: T = cell.volume * source_rate[cell.material] * time_step;
            total_weight_particles += cell_weight_particles;
        });
    });

    let n_particles = mcco.params.simulation_params.n_particles as usize;
    let source_fraction: T = FromPrimitive::from_f64(0.1).unwrap();
    let source_particle_weight: T = total_weight_particles
        / (source_fraction * FromPrimitive::from_usize(n_particles).unwrap());
    assert_ne!(source_particle_weight, zero());
    mcco.source_particle_weight = source_particle_weight.to_f64().unwrap();

    let vault_size = mcco.particle_vault_container.vault_size;
    let mut processing_idx = mcco.particle_vault_container.particles_processing_size() / vault_size;

    println!("n_domains: {}", mcco.domain.len());

    // on each domain
    mcco.domain
        .iter_mut()
        .enumerate()
        .for_each(|(domain_idx, dom)| {
            // we'll update the tally separately and merge data after
            // this allows for a read-only iterator
            let mut cell_source_tally: Vec<usize> = vec![0; dom.cell_state.len()];

            println!("{} cells in domain {}", dom.cell_state.len(), domain_idx);

            // on each cell
            dom.cell_state
                .iter()
                .enumerate()
                .for_each(|(cell_idx, cell)| {
                    let cell_weight_particle: T =
                        cell.volume * source_rate[cell.material] * time_step;

                    // floor/ceil it before cast ?
                    let cell_n_particles: usize = (cell_weight_particle / source_particle_weight)
                        .floor()
                        .to_usize()
                        .unwrap();
                    cell_source_tally[cell_idx] = cell.source_tally;
                    // create cell_n_particles and add them to the vaults
                    //println!("creating {cell_n_particles} particles in cell {cell_idx}");
                    (0..cell_n_particles).into_iter().for_each(|_ii| {
                        let mut particle: MCParticle<T> = MCParticle::default();
                        // atomic in original code
                        let mut rand_n_seed = cell_source_tally[cell_idx] as u64;
                        cell_source_tally[cell_idx] += 1;

                        rand_n_seed += cell.id as u64;

                        particle.random_number_seed = spawn_rn_seed::<T>(&mut rand_n_seed);
                        particle.identifier = rand_n_seed;

                        particle.coordinate = generate_coordinate_3dg(
                            &mut particle.random_number_seed,
                            dom,
                            cell_idx,
                        );

                        particle
                            .direction_cosine
                            .sample_isotropic(&mut particle.random_number_seed);

                        // sample energy uniformly in [emin; emax] MeV
                        let range: T = FromPrimitive::from_f64(
                            mcco.params.simulation_params.e_max
                                - mcco.params.simulation_params.e_min,
                        )
                        .unwrap();
                        let sample: T = rng_sample(&mut particle.random_number_seed);
                        particle.kinetic_energy = sample * range
                            + FromPrimitive::from_f64(mcco.params.simulation_params.e_min).unwrap();

                        let speed: T = speed_from_energy(particle.kinetic_energy);
                        particle.velocity.x = speed * particle.direction_cosine.alpha;
                        particle.velocity.y = speed * particle.direction_cosine.beta;
                        particle.velocity.z = speed * particle.direction_cosine.gamma;

                        particle.domain = domain_idx;
                        particle.cell = cell_idx;
                        particle.task = 0; // used task_idx in original code but it stayed const
                        particle.weight = source_particle_weight;

                        let rand_f = rng_sample(&mut particle.random_number_seed);
                        particle.time_to_census = time_step * rand_f;

                        let base_particle: MCBaseParticle<T> = MCBaseParticle::new(&particle);
                        mcco.particle_vault_container
                            .add_processing_particle(base_particle, &mut processing_idx);
                        //println!("added particle; total # in cell: {ii}");

                        // atomic in original code
                        mcco.tallies.balance_task[particle.task].source += 1;
                    });
                });
            // update source_tally
            (0..dom.cell_state.len()).into_iter().for_each(|cell_idx| {
                dom.cell_state[cell_idx].source_tally = cell_source_tally[cell_idx];
            });
        });
}

fn speed_from_energy<T: CustomFloat>(energy: T) -> T {
    let rest_mass_energy: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
    let speed_of_light: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();
    speed_of_light
        * (energy * (energy * two * (rest_mass_energy))
            / ((energy + rest_mass_energy) * (energy + rest_mass_energy)))
            .sqrt()
}
