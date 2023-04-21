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
use num::FromPrimitive;

fn main() {
    let cli = Cli::parse();

    let params: Parameters<f64> = Parameters::get_parameters(cli).unwrap();
    println!("Printing Parameters:\n{params:#?}");

    let n_steps = params.simulation_params.n_steps;

    let mcdata = init_mcdata(params);
    let mut containers = init_particle_containers(&mcdata.params, &mcdata.exec_info);
    let mut mcunits = init_mcunits(&mcdata);

    match mcdata.exec_info.exec_policy {
        ExecPolicy::Sequential => {
            let container = &mut containers[0];
            let mcunit = &mut mcunits[0];

            mc_fast_timer::start(&mut mcunit.fast_timer, Section::Main);

            for step in 0..n_steps {
                cycle_sync(&mcdata, mcunit, container, step);
                cycle_process(&mcdata, mcunit, container);
            }
            cycle_sync(&mcdata, mcunit, container, n_steps);

            mc_fast_timer::stop(&mut mcunit.fast_timer, Section::Main);
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
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
    step: usize,
) {
    if step != 0 {
        // Finalize after processing; centralize data at each step or just use as it progress?
        mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleFinalize);

        mcunit.tallies.balance_cycle.end = container.processed_particles.len() as u64;
        mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleFinalize);
        mcunit
            .tallies
            .print_summary(&mut mcunit.fast_timer, step - 1);
        mcunit
            .tallies
            .cycle_finalize(mcdata.params.simulation_params.coral_benchmark);
        mcunit.update_spectrum(container, mcdata);
        mcunit.fast_timer.clear_last_cycle_timers();
    }

    mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleInit);

    mcunit.clear_cross_section_cache();
    container.swap_processing_processed();

    mcunit.tallies.balance_cycle.start = container.processing_particles.len() as u64;

    // compute total weight of the problem to source correctly when using multiple units
    mcunit.update_unit_weight(mcdata); // call this on all units
    let total_problem_weight: T = mcunit.unit_weight; // sum on all units
    let source_particle_weight: T = total_problem_weight
        / (<T as FromPrimitive>::from_f64(SRC_FRACTION).unwrap()
            * FromPrimitive::from_u64(mcdata.params.simulation_params.n_particles).unwrap());

    population_control::source_now(mcdata, mcunit, container, source_particle_weight);
    population_control::population_control(mcdata, mcunit, container);
    population_control::roulette_low_weight_particles(
        mcdata.params.simulation_params.low_weight_cutoff,
        source_particle_weight,
        container,
        &mut mcunit.tallies.balance_cycle,
    );

    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleInit);
}

//==================================
// Processing core of the simulation
//==================================

pub fn cycle_process<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTracking);
    loop {
        while !container.test_done_new() {
            mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTrackingKernel);

            // track particles
            container
                .processing_particles
                .iter_mut()
                .for_each(|base_particle| {
                    cycle_tracking_guts(
                        mcdata,
                        mcunit,
                        base_particle,
                        &mut container.extra_particles,
                        &mut container.send_queue,
                    );
                    if base_particle.species != Species::Unknown {
                        container.processed_particles.push(base_particle.clone());
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
