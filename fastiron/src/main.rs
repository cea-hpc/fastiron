use std::iter::zip;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use clap::Parser;
use num::{one, zero, FromPrimitive};

use hwlocality::cpu::binding::CpuBindingFlags;
use hwlocality::object::types::ObjectType;
use hwlocality::object::TopologyObject;
use hwlocality::Topology;
use rayon::ThreadPoolBuilder;

use fastiron::constants::sim::SRC_FRACTION;
use fastiron::constants::CustomFloat;
use fastiron::data::tallies::TalliedEvent;
use fastiron::init::{init_mcdata, init_mcunits, init_particle_containers, init_results};
use fastiron::montecarlo::{MonteCarloData, MonteCarloResults, MonteCarloUnit};
use fastiron::parameters::Parameters;
use fastiron::particles::particle_container::ParticleContainer;
use fastiron::simulation::population_control;
use fastiron::utils::coral_benchmark_correctness::coral_benchmark_correctness;
use fastiron::utils::input::Cli;
use fastiron::utils::mc_fast_timer::{self, Section};
use fastiron::utils::mc_processor_info::ExecPolicy;

//================================
// Which float type are we using ?
//================================

#[cfg(feature = "single-precision")]
type FloatType = f32;

#[cfg(not(feature = "single-precision"))]
type FloatType = f64;

//=====
// Main
//=====

fn main() {
    let cli = Cli::parse();

    let params: Parameters<FloatType> = Parameters::get_parameters(cli).unwrap();
    println!("[Simulation Parameters]\n{:#?}", params.simulation_params);

    let n_cells_tot =
        params.simulation_params.nx * params.simulation_params.ny * params.simulation_params.nz;

    if params.simulation_params.n_particles as usize / n_cells_tot < 10 {
        println!("[ERROR] TOO FEW PARTICLES PER CELL OVERALL");
        println!("[ERROR] Need at least 10 particles per cell of the mesh overall");
        return;
    }

    //===============
    // Initialization
    //===============

    let start_init = Instant::now();
    println!("[Initialization]: Start");

    let n_steps = params.simulation_params.n_steps;

    let mut mcdata = init_mcdata(params);
    let mut containers = init_particle_containers(&mcdata.params, &mcdata.exec_info);
    let mut mcunits = init_mcunits(&mcdata);
    let mut mcresults = init_results(&mcdata.params);

    // rayon only => one global thread pool
    if mcdata.exec_info.exec_policy == ExecPolicy::Rayon {
        // custom thread-pool init in this case
        if mcdata.exec_info.n_rayon_threads != 0 {
            let topo = Arc::new(Mutex::new(Topology::new().unwrap()));
            ThreadPoolBuilder::new()
                .num_threads(mcdata.exec_info.n_rayon_threads)
                .start_handler(move |thread_id| bind_threads(thread_id, &topo))
                .build_global()
                .unwrap();
        }
        mcdata.exec_info.n_rayon_threads = rayon::current_num_threads();
    }

    println!("[Initialization]: Done");
    println!(
        "[Initialization]: {}ms elapsed",
        start_init.elapsed().as_millis()
    );

    println!("[Execution Info]");
    println!("{}", mcdata.exec_info);

    //==========
    // Core loop
    //==========

    match mcdata.exec_info.exec_policy {
        // single unit
        ExecPolicy::Sequential | ExecPolicy::Rayon => {
            mc_fast_timer::start(&mut mcunits[0].fast_timer, Section::Main);

            for step in 0..n_steps {
                cycle_sync(
                    &mut mcdata,
                    &mut mcunits,
                    &mut containers,
                    step,
                    &mut mcresults,
                );
                cycle_process(&mcdata, &mut mcunits[0], &mut containers[0]);
            }
            cycle_sync(
                &mut mcdata,
                &mut mcunits,
                &mut containers,
                n_steps,
                &mut mcresults,
            );

            mc_fast_timer::stop(&mut mcunits[0].fast_timer, Section::Main);
        }
        // multiple units
        ExecPolicy::Distributed | ExecPolicy::Hybrid => todo!(),
    }

    //========
    // Wrap-up
    //========

    game_over(&mcdata, &mut mcunits, &mcresults);

    coral_benchmark_correctness(&mcresults);
}

//==========================
// End of simulation cleanup
//==========================

pub fn game_over<T: CustomFloat>(
    mcdata: &MonteCarloData<T>,
    mcunits: &mut [MonteCarloUnit<T>],
    mcresults: &MonteCarloResults<T>,
) {
    match mcdata.exec_info.exec_policy {
        ExecPolicy::Sequential | ExecPolicy::Rayon => {
            let mcunit = &mut mcunits[0];
            mcunit.fast_timer.update_main_stats();

            mcunit.fast_timer.cumulative_report(
                mcresults.balance_cumulative[TalliedEvent::NumSegments],
                mcdata.params.simulation_params.csv,
            );

            mcresults.spectrum.print_spectrum(mcdata);
        }
        ExecPolicy::Distributed | ExecPolicy::Hybrid => todo!(),
    }
}

//========================
// Thread binding routines
//========================

pub fn bind_threads(thread_id: usize, topo: &Arc<Mutex<Topology>>) {
    // get thread id
    let pthread_id = unsafe { libc::pthread_self() };
    // get cpu topology
    let locked_topo = topo.lock().unwrap();
    // get current thread's cpu affinity
    let cpu_set = {
        let ancestor_lvl = locked_topo
            .depth_or_above_for_type(ObjectType::NUMANode)
            .unwrap_or_default();
        let mut targets = locked_topo.objects_at_depth(ancestor_lvl);
        let ancestor = targets.next().expect("No common ancestor found");
        let processing_units = locked_topo.objects_with_type(ObjectType::PU);
        let unit = processing_units
            .into_iter()
            .filter(|pu| has_ancestor(pu, ancestor))
            .cycle()
            .nth(thread_id)
            .expect("No cores below given ancestor");
        unit.cpuset().unwrap()
    };

    match locked_topo.bind_thread_cpu(pthread_id, cpu_set, CpuBindingFlags::THREAD) {
        Ok(_) => {}
        Err(e) => {
            println!("[Error]: Could not bind threads to cpu cores:");
            println!("{e:#?}");
        }
    }
}

fn has_ancestor(object: &TopologyObject, ancestor: &TopologyObject) -> bool {
    let father = object.parent();
    father
        .map(|f| {
            (f.object_type() == ancestor.object_type()
                && f.logical_index() == ancestor.logical_index())
                || has_ancestor(f, ancestor)
        })
        .unwrap_or(false)
}

//============================
// Sync node of the simulation
//============================

pub fn cycle_sync<T: CustomFloat>(
    mcdata: &mut MonteCarloData<T>,
    mcunits: &mut [MonteCarloUnit<T>],
    containers: &mut [ParticleContainer<T>],
    step: usize,
    mcresults: &mut MonteCarloResults<T>,
) {
    mc_fast_timer::start(&mut mcunits[0].fast_timer, Section::CycleSync);

    if step != 0 {
        // Finalize after processing; centralize data at each step or just use as it progress?

        match mcdata.exec_info.exec_policy {
            ExecPolicy::Sequential | ExecPolicy::Rayon => {
                // if sequential/rayon-only, just use the single Monte-Carlo unit
                mcunits[0].tallies.balance_cycle[TalliedEvent::End] =
                    containers[0].processed_particles.len() as u64;
                mcunits[0].tallies.print_summary(
                    &mut mcunits[0].fast_timer,
                    step - 1,
                    mcdata.params.simulation_params.csv,
                );
                mcresults.update_stats(mcunits);
                mcresults.update_spectrum(containers);
                mcunits[0].fast_timer.clear_last_cycle_timers();
            }
            ExecPolicy::Distributed | ExecPolicy::Hybrid => todo!(), // need to reduce
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
        mcunit.tallies.balance_cycle[TalliedEvent::Start] = local_n_particles as u64;
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
    // compute split factor
    let split_rr_factor: T = population_control::compute_split_factor(
        mcdata.params.simulation_params.n_particles as usize,
        mcdata.global_n_particles,
        container.processing_particles.len(),
        mcdata.exec_info.n_units,
        mcdata.params.simulation_params.load_balance,
    );
    // regulate accordingly
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

    while !container.is_done_processing() {
        // sort particles
        mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTrackingSort);
        container.sort_processing();
        mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTrackingSort);

        // track particles
        mc_fast_timer::start(&mut mcunit.fast_timer, Section::CycleTrackingProcess);
        container.process_particles(mcdata, mcunit);
        mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTrackingProcess);

        // clean extra here
        container.clean_extra_vaults();
    }

    mc_fast_timer::stop(&mut mcunit.fast_timer, Section::CycleTracking);
}
