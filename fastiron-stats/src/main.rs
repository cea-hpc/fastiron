use std::fs::File;

use fastiron_stats::{
    io::command_line::Cli,
    structures::{
        processed::{ComparisonResults, CorrelationResults, ScalingResults, ScalingType},
        raw::{TalliesReport, TimerReport},
    },
};

use clap::Parser;

fn main() {
    // Input handling
    let cli = Cli::parse();

    if let Some(filenames) = cli.comparison {
        println!("Comparing timers...");
        // Get data, process it, save results
        let old_timer_report = TimerReport::from(File::open(&filenames[0]).unwrap());
        let new_timer_report = TimerReport::from(File::open(&filenames[1]).unwrap());
        let results = ComparisonResults::from((old_timer_report, new_timer_report));
        results.save();
        println!("Done!");

        if cli.plot {
            results.plot();
            println!("Plotted results");
        }
    }

    if let Some(tallies_report) = cli.correlation {
        println!("Processing tallied data...");
        // Get data, process it, save results
        let tallies_data = TalliesReport::from(File::open(tallies_report).unwrap());
        let results = CorrelationResults::from(tallies_data);
        results.save();
        println!("Done!");

        if cli.plot {
            results.plot();
            println!("Plotted results");
        }
    }

    if let Some(root_path) = cli.weak_scaling_root {
        println!("Processing weak scaling data...");
        // Get data, process it, save results
        let results =
            ScalingResults::from((root_path.as_str(), &cli.scaling_params, ScalingType::Weak));
        results.save();
        println!("Done!");

        if cli.plot {
            results.plot();
            println!("Plotted results");
        }
    }

    if let Some(root_path) = cli.strong_scaling_root {
        println!("Processing strong scaling data...");
        let results =
            ScalingResults::from((root_path.as_str(), &cli.scaling_params, ScalingType::Strong));
        results.save();
        println!("Done!");

        if cli.plot {
            results.plot();
            println!("Plotted results");
        }
    }
    println!("Finished! All data is ready for use.")
}
