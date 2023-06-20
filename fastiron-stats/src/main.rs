use std::fs::File;

use fastiron_stats::{
    io::{
        command_line::Cli,
        files::{
            compile_scaling_data, get_scaling_data, read_tallies, save_percents,
            save_popsync_results, save_tracking_results,
        },
    },
    processing::{self, compare},
    structures::raw::TimerReport,
};

use clap::Parser;

fn main() {
    // Input handling
    let cli = Cli::parse();

    if let Some(filenames) = cli.comparison {
        println!("Comparing timers...");
        // Get data, process it, save results
        let old_timers = &filenames[0];
        let new_timers = &filenames[1];
        let old_timer_report = TimerReport::from(File::open(old_timers).unwrap());
        let new_timer_report = TimerReport::from(File::open(new_timers).unwrap());
        let percents = compare(&old_timer_report, &new_timer_report);
        save_percents(old_timer_report, new_timer_report, &percents);
        println!("Done!");

        if cli.plot {
            // plot results

            println!("Plotted results");
        }
    }

    if let Some(tallies_report) = cli.correlation {
        println!("Processing tallied data...");
        // Get data, process it, save results
        let tallies_data = read_tallies(&tallies_report);
        let tracking_res = processing::build_tracking_results(&tallies_data);
        let popsync_res = processing::build_popsync_results(&tallies_data);
        save_tracking_results(&tracking_res);
        save_popsync_results(&popsync_res);
        println!("Done!");

        if cli.plot {
            // plot results

            println!("Plotted results");
        }
    }

    if let Some(root_path) = cli.weak_scaling_root {
        println!("Processing weak scaling data...");
        // Get data, process it, save results
        /*
        let timers = get_scaling_data(root, n_start, step, n_iter, progression);
        compile_scaling_data(&timers);
        */
        println!("Done!");

        if cli.plot {
            // plot results

            println!("Plotted results");
        }
    }

    if let Some(root_path) = cli.strong_scaling_root {
        println!("Processing strong scaling data...");
        // Get data, process it, save results
        /*
        let timers = get_scaling_data(root, n_start, step, n_iter, progression);
        compile_scaling_data(&timers);
        */
        println!("Done!");

        if cli.plot {
            // plot results

            println!("Plotted results");
        }
    }
    println!("Finished! All data is ready for use.")
}
