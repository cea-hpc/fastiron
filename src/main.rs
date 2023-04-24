use clap::Parser;
use fastiron::constants::sim::SRC_FRACTION;
use fastiron::constants::CustomFloat;
use fastiron::init::{init_mcdata, init_mcunits, init_particle_containers};
use fastiron::montecarlo::{MonteCarloData, MonteCarloUnit};
use fastiron::parameters::Parameters;
use fastiron::particles::mc_particle::Species;
use fastiron::particles::particle_container::ParticleContainer;
use fastiron::simulation::cycle_tracking::cycle_tracking_guts;
use fastiron::simulation::population_control;
use fastiron::utils::coral_benchmark_correctness::coral_benchmark_correctness;
use fastiron::utils::io_utils::Cli;
use fastiron::utils::mc_fast_timer::{self, Section};
use fastiron::utils::mc_processor_info::ExecPolicy;
use num::{one, FromPrimitive};

fn main() {
    let cli = Cli::parse();

    let params: Parameters<f64> = Parameters::get_parameters(cli).unwrap();
    println!("Printing Parameters:\n{params:#?}");

    let n_steps = params.simulation_params.n_steps;

    let mut mcdata = init_mcdata(params);
    let mut containers = init_particle_containers(&mcdata.params, &mcdata.exec_info);
    let mut mcunits = init_mcunits(&mcdata);

    match mcdata.exec_info.exec_policy {
        ExecPolicy::Sequential => {
            //let container = &mut containers[0];
            //let mcunit = &mut mcunits[0];

            mc_fast_timer::start(&mut mcunits[0].fast_timer, Section::Main);

            for step in 0..n_steps {
                cycle_sync(&mut mcdata, &mut mcunits, &mut containers, step);
                cycle_process(&mcdata, &mut mcunits[0], &mut containers[0]);
            }
            cycle_sync(&mut mcdata, &mut mcunits, &mut containers, n_steps);

            mc_fast_timer::stop(&mut mcunits[0].fast_timer, Section::Main);
        }
        ExecPolicy::Parallel => {
            todo!()
        }
    }

    game_over(&mcdata, &mut mcunits[0]);

    coral_benchmark_correctness(
        mcdata.params.simulation_params.coral_benchmark,
        &mcunits[0].tallies,
    );
}

pub fn game_over<T: CustomFloat>(mcdata: &MonteCarloData<T>, mcunit: &mut MonteCarloUnit<T>) {
    mcunit.fast_timer.update_main_stats();

    mcunit
        .fast_timer
        .cumulative_report(mcunit.tallies.balance_cumulative.num_segments);

    mcunit.tallies.spectrum.print_spectrum(mcdata);
}

//============================
// Sync node of the simulation
//============================

pub fn cycle_sync<T: CustomFloat>(
    mcdata: &mut MonteCarloData<T>,
    mcunits: &mut [MonteCarloUnit<T>],
    containers: &mut [ParticleContainer<T>],
    step: usize,
) {
    if step != 0 {
        // Finalize after processing; centralize data at each step or just use as it progress?

        match mcdata.exec_info.exec_policy {
            ExecPolicy::Sequential => {
                // if sequential, just use the single Monte-Carlo unit
                mcunits[0].tallies.balance_cycle.end =
                    containers[0].processed_particles.len() as u64;
                mcunits[0]
                    .tallies
                    .print_summary(&mut mcunits[0].fast_timer, step - 1);
                mcunits[0]
                    .tallies
                    .cycle_finalize(mcdata.params.simulation_params.coral_benchmark);
                mcunits[0].tallies.update_spectrum(&containers[0]);
                mcunits[0].fast_timer.clear_last_cycle_timers();
            }
            ExecPolicy::Parallel => {
                todo!()
            }
        }

        if step == mcdata.params.simulation_params.n_steps + 1 {
            return;
        }
    }

    // compute total weight of the problem to source correctly when using multiple units
    let total_problem_weight: T = mcunits
        .iter_mut()
        .map(|mcunit| {
            mcunit.update_unit_weight(mcdata);
            mcunit.unit_weight
        })
        .sum();
    let n_particles_to_spawn: T = <T as FromPrimitive>::from_f64(SRC_FRACTION).unwrap()
        * FromPrimitive::from_u64(mcdata.params.simulation_params.n_particles).unwrap();
    mcdata.source_particle_weight = total_problem_weight / n_particles_to_spawn;
}

//==================================
// Processing core of the simulation
//==================================

pub fn cycle_process<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    mcunit.clear_cross_section_cache();
    container.swap_processing_processed();
    mcunit.tallies.balance_cycle.start = container.processing_particles.len() as u64;

    population_control::source_now(mcdata, mcunit, container);
    let split_rr_factor: T = population_control::compute_split_factor(
        container,
        mcdata.params.simulation_params.n_particles as usize,
        mcdata.exec_info.num_threads,
        mcdata.params.simulation_params.load_balance,
    );
    if split_rr_factor != one() {
        population_control::regulate(
            split_rr_factor,
            container,
            &mut mcunit.tallies.balance_cycle,
        )
    }
    population_control::roulette_low_weight_particles(
        mcdata.params.simulation_params.low_weight_cutoff,
        mcdata.source_particle_weight,
        container,
        &mut mcunit.tallies.balance_cycle,
    );

    mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTracking);
    loop {
        while !container.test_done_new() {
            mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTrackingKernel);

            // track particles
            container
                .processing_particles
                .iter_mut()
                .for_each(|particle| {
                    cycle_tracking_guts(
                        mcdata,
                        mcunit,
                        particle,
                        &mut container.extra_particles,
                        &mut container.send_queue,
                    );
                    if particle.species != Species::Unknown {
                        container.processed_particles.push(particle.clone());
                    }
                });
            container.processing_particles.clear();

            mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTrackingKernel);
            mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTrackingComm);

            // clean extra here
            container.process_sq();
            container.clean_extra_vaults();

            mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTrackingComm);
        }
        if container.test_done_new() {
            break;
        }
    }
    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTracking);
}
