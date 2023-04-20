//! Code regulating the number of particles in the simulation
//!
//! This module contains the three main functions used to regulate the number
//! of particles in the simulation as well as two internal functions used by
//! those.

use num::{one, zero, FromPrimitive};

use crate::{
    constants::{
        physical::{LIGHT_SPEED, NEUTRON_REST_MASS_ENERGY},
        CustomFloat,
    },
    data::{direction_cosine::DirectionCosine, tallies::Balance},
    montecarlo::{MonteCarloData, MonteCarloUnit},
    particles::{mc_base_particle::MCBaseParticle, particle_container::ParticleContainer},
    simulation::mct::generate_coordinate_3dg,
    utils::mc_rng_state::{rng_sample, spawn_rn_seed},
};

/// Routine used to monitor and regulate population level.
///
/// If load balancing is enabled, the spawned particle will be spread
/// throughout the processors. Using the current number of particle and
/// the target number of particles, the function computes a split factor.
/// If the split factor is strictly below one, there are too many particles,
/// if it is striclty superior to one, there are too little. Particles are
/// then either randomly killed or spawned to get to the desired number.
pub fn population_control<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    let mut target_n_particles: usize = mcdata.params.simulation_params.n_particles as usize;
    let mut global_n_particles: usize = 0;
    let local_n_particles: usize = container.processing_particles.len();
    let load_balance = mcdata.params.simulation_params.load_balance;

    if load_balance {
        // Spread the target number of particle among the processors
        let tmp: T = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
            / FromPrimitive::from_usize(mcdata.exec_info.num_threads).unwrap();
        target_n_particles = tmp.ceil().to_usize().unwrap();
    } else {
        global_n_particles = local_n_particles;
    }

    let mut split_rr_factor: T = one();
    if load_balance {
        // Compute processor-specific split factor
        if local_n_particles != 0 {
            split_rr_factor = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
                / FromPrimitive::from_usize(local_n_particles).unwrap();
        }
    } else {
        split_rr_factor = <T as FromPrimitive>::from_usize(target_n_particles).unwrap()
            / FromPrimitive::from_usize(global_n_particles).unwrap();
    }

    if split_rr_factor != one() {
        population_control_guts(
            split_rr_factor,
            container,
            &mut mcunit.tallies.balance_cycle,
        );
    }
}

fn population_control_guts<T: CustomFloat>(
    split_rr_factor: T,
    container: &mut ParticleContainer<T>,
    balance: &mut Balance,
) {
    if split_rr_factor < one() {
        // too many particles; roll for a kill
        container.processing_particles.retain_mut(|pp| {
            let rand_f: T = rng_sample(&mut pp.random_number_seed);
            if rand_f > split_rr_factor {
                // particle dies
                balance.rr += 1;
                false
            } else {
                // particle survives with increased weight
                pp.weight /= split_rr_factor;
                true
            }
        });
    } else if split_rr_factor > one() {
        // not enough particles; create new ones by splitting
        container.processing_particles.iter_mut().for_each(|pp| {
            let rand_f: T = rng_sample(&mut pp.random_number_seed);
            let mut split_factor = split_rr_factor.floor();
            if rand_f > split_rr_factor - split_factor {
                split_factor -= one();
            }
            pp.weight /= split_rr_factor;

            let n_split: usize = split_factor.to_usize().unwrap();
            (0..n_split).for_each(|_| {
                let mut split_pp = pp.clone();
                balance.split += 1;
                split_pp.random_number_seed = spawn_rn_seed::<T>(&mut pp.random_number_seed);
                split_pp.identifier = split_pp.random_number_seed;

                container.extra_particles.push(split_pp);
            })
        });
        container.clean_extra_vaults();
    }
}

/// Play russian-roulette with low-weight particles relative
/// to the source particle weight.
///
/// This function regulates the number of low (statistica) weight particle to
/// prevent clusters of low energy particle from falsifying the results.
pub fn roulette_low_weight_particles<T: CustomFloat>(
    low_weight_cutoff: T,
    source_particle_weight: T,
    container: &mut ParticleContainer<T>,
    balance: &mut Balance,
) {
    if low_weight_cutoff > zero() {
        let weight_cutoff = low_weight_cutoff * source_particle_weight;

        container.processing_particles.retain_mut(|pp| {
            if pp.weight <= weight_cutoff {
                let rand_f: T = rng_sample(&mut pp.random_number_seed);
                if rand_f <= low_weight_cutoff {
                    // particle survives with increased weight
                    pp.weight /= low_weight_cutoff;
                    true
                } else {
                    // particle dies
                    balance.rr += 1;
                    false
                }
            } else {
                // particle survives
                true
            }
        });
    }
}

/// Simulates the sources according to the problem's parameters.
///
/// This function spawns particle is source regions. Each time this function
/// is called (once per cycle), 10% of the target number of particles are
/// spawned. _Where_ they are spawned depends on both deterministic factors and
/// randomness.
pub fn source_now<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    let time_step = mcdata.params.simulation_params.dt;

    // this is a constant; add it to mcco ?
    let mut source_rate: Vec<T> = vec![zero(); mcdata.material_database.mat.len()];
    source_rate
        .iter_mut()
        .zip(mcdata.material_database.mat.iter())
        .for_each(|(lhs, mat)| {
            let rhs = mcdata.params.material_params[&mat.name].source_rate;
            *lhs = rhs;
        });

    let mut total_weight_particles: T = zero();
    mcunit.domain.iter().for_each(|dom| {
        dom.cell_state.iter().for_each(|cell| {
            // constant because cell volume is constant in our program
            let cell_weight_particles: T = cell.volume * source_rate[cell.material] * time_step;
            total_weight_particles += cell_weight_particles;
        });
    });

    let n_particles = mcdata.params.simulation_params.n_particles as usize;

    let source_fraction: T = FromPrimitive::from_f64(0.1).unwrap();

    let source_particle_weight: T = total_weight_particles
        / (source_fraction * FromPrimitive::from_usize(n_particles).unwrap());
    assert_ne!(source_particle_weight, zero());

    mcunit.source_particle_weight = source_particle_weight;

    // on each domain
    mcunit
        .domain
        .iter_mut()
        .enumerate()
        .for_each(|(domain_idx, dom)| {
            // on each cell
            dom.cell_state
                .iter_mut()
                .enumerate()
                .for_each(|(cell_idx, cell)| {
                    // compute number of particles to be created in the cell
                    let cell_weight_particle: T =
                        cell.volume * source_rate[cell.material] * time_step;
                    let cell_n_particles: usize = (cell_weight_particle / source_particle_weight)
                        .floor()
                        .to_usize()
                        .unwrap();

                    // create cell_n_particles and add them to the vaults
                    let sourced = (0..cell_n_particles).map(|_| {
                        let mut base_particle: MCBaseParticle<T> = MCBaseParticle::default();

                        // atomic in original code
                        let mut rand_n_seed = cell.source_tally as u64;
                        cell.source_tally += 1;

                        rand_n_seed += cell.id as u64;

                        base_particle.random_number_seed = spawn_rn_seed::<T>(&mut rand_n_seed);
                        base_particle.identifier = rand_n_seed;

                        base_particle.coordinate = generate_coordinate_3dg(
                            &mut base_particle.random_number_seed,
                            &dom.mesh,
                            cell_idx,
                            cell.volume,
                        );
                        let mut direction_cosine = DirectionCosine::default();
                        direction_cosine.sample_isotropic(&mut base_particle.random_number_seed);

                        // sample energy uniformly in [emin; emax] MeV
                        let range = mcdata.params.simulation_params.e_max
                            - mcdata.params.simulation_params.e_min;
                        let sample: T = rng_sample(&mut base_particle.random_number_seed);
                        base_particle.kinetic_energy =
                            sample * range + mcdata.params.simulation_params.e_min;

                        let speed: T = speed_from_energy(base_particle.kinetic_energy);
                        base_particle.velocity = direction_cosine.dir * speed;

                        base_particle.domain = domain_idx;
                        base_particle.cell = cell_idx;
                        base_particle.weight = source_particle_weight;

                        let mut rand_f: T = rng_sample(&mut base_particle.random_number_seed);
                        base_particle.num_mean_free_paths = -one::<T>() * rand_f.ln();
                        rand_f = rng_sample(&mut base_particle.random_number_seed);
                        base_particle.time_to_census = time_step * rand_f;

                        // atomic in original code
                        mcunit.tallies.balance_cycle.source += 1;

                        base_particle
                    });
                    container.processing_particles.extend(sourced);
                });
        });
}

fn speed_from_energy<T: CustomFloat>(energy: T) -> T {
    let rest_mass_energy: T = FromPrimitive::from_f64(NEUTRON_REST_MASS_ENERGY).unwrap();
    let speed_of_light: T = FromPrimitive::from_f64(LIGHT_SPEED).unwrap();
    let two: T = FromPrimitive::from_f64(2.0).unwrap();
    speed_of_light
        * (energy * (energy + two * (rest_mass_energy))
            / ((energy + rest_mass_energy) * (energy + rest_mass_energy)))
            .sqrt()
}

//=============
// Unit tests
//=============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_speed_from_e() {
        let energy = 15.032;
        let speed = speed_from_energy(energy);
        assert_eq!(speed, 5299286790.50638);
    }
}
