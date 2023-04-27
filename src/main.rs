use std::iter::zip;
use std::time::Instant;

use clap::Parser;
use fastiron::constants::sim::SRC_FRACTION;
use fastiron::constants::CustomFloat;
use fastiron::init::{init_mcdata, init_mcunits, init_particle_containers};
use fastiron::montecarlo::{MonteCarloData, MonteCarloUnit};
use fastiron::parameters::Parameters;
use fastiron::particles::particle_container::ParticleContainer;
use fastiron::simulation::population_control;
use fastiron::utils::coral_benchmark_correctness::coral_benchmark_correctness;
use fastiron::utils::io_utils::Cli;
use fastiron::utils::mc_fast_timer::{self, Section};
use fastiron::utils::mc_processor_info::ExecPolicy;
use num::{one, zero, FromPrimitive};

fn main() {
    let cli = Cli::parse();

    let params: Parameters<f64> = Parameters::get_parameters(cli).unwrap();
    println!("[Simulation Parameters]\n{:#?}", params.simulation_params);
    let start_init = Instant::now();
    println!("[Initialization]: Start");

    let n_steps = params.simulation_params.n_steps;

    let mut mcdata = init_mcdata(params);
    let mut containers = init_particle_containers(&mcdata.params, &mcdata.exec_info);
    let mut mcunits = init_mcunits(&mcdata);

    println!("[Initialization]: Done");
    println!(
        "[Initialization]: {}ms elapsed",
        start_init.elapsed().as_millis()
    );

    match mcdata.exec_info.exec_policy {
        ExecPolicy::Sequential => {
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
    mc_fast_timer::start(&mut mcunits[0].fast_timer, Section::CycleSync);

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
                // centralize / finalize
                todo!()
            }
        }

        if step == mcdata.params.simulation_params.n_steps + 1 {
            mc_fast_timer::stop(&mut mcunits[0].fast_timer, Section::CycleSync);
            return;
        }
    }

    // prepare structures for next processing cycle
    let iter = zip(mcunits.iter_mut(), containers.iter_mut());
    let mut current_n_particles: usize = 0;
    let mut total_problem_weight: T = zero();
    iter.for_each(|(mcunit, container)| {
        mcunit.update_unit_weight(mcdata);
        mcunit.clear_cross_section_cache();
        container.swap_processing_processed();
        let local_n_particles = container.processing_particles.len();
        mcunit.tallies.balance_cycle.start = local_n_particles as u64;
        current_n_particles += local_n_particles;
        total_problem_weight += mcunit.unit_weight;
    });
    let n_particles_to_spawn: T = <T as FromPrimitive>::from_f64(SRC_FRACTION).unwrap()
        * FromPrimitive::from_u64(mcdata.params.simulation_params.n_particles).unwrap();
    mcdata.source_particle_weight = total_problem_weight / n_particles_to_spawn;
    // current number of particle + the one that will be sourced asap
    // i.e. number of particles before population control:
    mcdata.global_n_particles = current_n_particles + n_particles_to_spawn.to_usize().unwrap();

    mc_fast_timer::stop(&mut mcunits[0].fast_timer, Section::CycleSync);
}

//==================================
// Processing core of the simulation
//==================================

pub fn cycle_process<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    mc_fast_timer::start(&mut mcunit.fast_timer, Section::PopulationControl);

    // source 10% of target number of particles
    population_control::source_now(mcdata, mcunit, container);
    // compute split factor & regulate accordingly; regulation include the low weight rr
    let split_rr_factor: T = population_control::compute_split_factor(
        mcdata.params.simulation_params.n_particles as usize,
        mcdata.global_n_particles,
        container.processing_particles.len(),
        mcdata.exec_info.num_threads,
        mcdata.params.simulation_params.load_balance,
    );
    if split_rr_factor < one() {
        container.regulate_population(
            split_rr_factor,
            mcdata.params.simulation_params.low_weight_cutoff,
            mcdata.source_particle_weight,
            &mut mcunit.tallies.balance_cycle,
        );
    } else if split_rr_factor > one() {
        container.split_population(
            split_rr_factor,
            mcdata.params.simulation_params.low_weight_cutoff,
            mcdata.source_particle_weight,
            &mut mcunit.tallies.balance_cycle,
        )
    }

    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::PopulationControl);
    mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTracking);

    loop {
        while !container.test_done_new() {
            mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTrackingKernel);

            // track particles
            container.process_particles(mcdata, mcunit);

            mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTrackingKernel);
            mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTrackingComm);

            // clean extra here
            container.process_sq();
            container.clean_extra_vaults();

            mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTrackingComm);
        }
        // change this if in parallel to also check pending/buffered particles
        if container.test_done_new() {
            break;
        }
    }

    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTracking);
}
