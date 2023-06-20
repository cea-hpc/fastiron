use fastiron_stats::{
    io::{
        command_line::Cli,
        files::{
            compile_scaling_data, get_scaling_data, read_tallies, read_timers, save_percents,
            save_popsync_results, save_tracking_results,
        },
    },
    processing::{self, compare},
};

use clap::Parser;

fn main() {
    // Input handling
    let cli = Cli::parse();

    if let Some(filenames) = cli.comparison {
        let old_timers = &filenames[0];
        let new_timers = &filenames[1];

        // Get data, process it, save results
        let old_timer_report = read_timers(old_timers);
        let new_timer_report = read_timers(new_timers);
        let percents = compare(old_timer_report, new_timer_report);
        save_percents(&percents);
    }

    if let Some(tallies_report) = cli.correlation {
        // Get data, process it, save results
        let tallies_data = read_tallies(&tallies_report);
        let tracking_res = processing::build_tracking_results(&tallies_data);
        let popsync_res = processing::build_popsync_results(&tallies_data);
        save_tracking_results(&tracking_res);
        save_popsync_results(&popsync_res);
    }

    if let Some(root_path) = cli.weak_scaling_root {
        // Get data, process it, save results
        /*
        let timers = get_scaling_data(root, n_start, step, n_iter, progression);
        compile_scaling_data(&timers);
        */
    }

    if let Some(root_path) = cli.strong_scaling_root {
        // Get data, process it, save results
        /*
        let timers = get_scaling_data(root, n_start, step, n_iter, progression);
        compile_scaling_data(&timers);
        */
    }
    println!("Finished! All data is ready for use.")
}
