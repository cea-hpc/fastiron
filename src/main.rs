use clap::Parser;
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
                cycle_init(&mcdata, mcunit, container);
                cycle_tracking(&mcdata, mcunit, container);
                cycle_finalize(&mcdata, mcunit, container, step);
            }

            mc_fast_timer::stop(&mut mcunit.fast_timer, Section::Main);
        }
        ExecPolicy::Parallel => {
            todo!()
        }
    }

    game_over(&mcdata, &mut mcunits[0]);

    // need to sum all tallies or already done before when in parallel?
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

pub fn cycle_init<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleInit);

    mcunit.clear_cross_section_cache();

    container.swap_processing_processed();

    let tmp = container.processing_particles.len() as u64;
    mcunit.tallies.balance_cycle.start = tmp;

    population_control::source_now(mcdata, mcunit, container);

    population_control::population_control(mcdata, mcunit, container);

    population_control::roulette_low_weight_particles(
        mcdata.params.simulation_params.low_weight_cutoff,
        mcunit.source_particle_weight,
        container,
        &mut mcunit.tallies.balance_cycle,
    );

    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleInit);
}

pub fn cycle_tracking<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
) {
    mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTracking);
    let mut done = false;
    loop {
        while !done {
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

            done = container.test_done_new();

            mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTrackingComm);
        }
        done = container.test_done_new();

        if done {
            break;
        }
    }
    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTracking);
}

pub fn cycle_finalize<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunit: &mut MonteCarloUnit<T>,
    container: &mut ParticleContainer<T>,
    step: usize,
) {
    mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleFinalize);

    // prepare data for summary; this would be a sync phase in parallel exec
    mcunit.tallies.balance_cycle.end = container.processed_particles.len() as u64;

    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleFinalize);

    // print summary
    mcunit.tallies.print_summary(&mut mcunit.fast_timer, step);

    // record / process data for the next cycle
    mcunit
        .tallies
        .cycle_finalize(mcdata.params.simulation_params.coral_benchmark);
    mcunit.update_spectrum(container, mcdata);

    mcunit.fast_timer.clear_last_cycle_timers();
}
