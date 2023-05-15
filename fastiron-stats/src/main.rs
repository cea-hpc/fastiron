use std::io::{self, stdout, Write};

use fastiron_stats::{
    io_utils::{
        compile_scaling_data, get_scaling_data, read_tallies, read_timers, save_percents,
        save_popsync_results, save_tracking_results,
    },
    processing::{self, compare},
    structures::ProgressionType,
};

fn main() {
    // Input handling
    let mut txt_input = String::new();

    while (txt_input.trim() != "y") & (txt_input.trim() != "n") {
        txt_input.clear();
        print!("Version comparison? (y/n): ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
    }
    let comparison = txt_input.trim() == "y";

    txt_input.clear();
    while (txt_input.trim() != "y") & (txt_input.trim() != "n") {
        txt_input.clear();
        print!("Correlation study? (y/n): ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
    }
    let correlation = txt_input.trim() == "y";

    txt_input.clear();
    while (txt_input.trim() != "y") & (txt_input.trim() != "n") {
        txt_input.clear();
        print!("Scaling study? (y/n): ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
    }
    let scaling = txt_input.trim() == "y";

    txt_input.clear();

    if comparison {
        println!("+---------------------------------------+");
        println!("|Performance Comparison Between Versions|");
        println!("+---------------------------------------+");
        // Get old report file
        print!("Old timers report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
        let old_timers = txt_input.trim().to_owned();
        txt_input.clear();
        // Get new report file
        print!("New timers report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
        let new_timers = txt_input.trim().to_owned();
        txt_input.clear();

        // Get data, process it, save results
        let old_timer_report = read_timers(&old_timers);
        let new_timer_report = read_timers(&new_timers);
        let percents = compare(old_timer_report, new_timer_report);
        save_percents(&percents);
    }

    if correlation {
        println!("+-----------------+");
        println!("|Correlation Study|");
        println!("+-----------------+");
        // Get tallied data
        print!("Tallies report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
        let tallies_report = txt_input.trim().to_owned();
        txt_input.clear();

        // Get data, process it, save results
        let tallies_data = read_tallies(&tallies_report);
        let tracking_res = processing::build_tracking_results(&tallies_data);
        let popsync_res = processing::build_popsync_results(&tallies_data);
        save_tracking_results(&tracking_res);
        save_popsync_results(&popsync_res);
    }

    if scaling {
        println!("+-------------+");
        println!("|Scaling Study|");
        println!("+-------------+");
        // Get naming root
        print!("Name root of the timers report .csv file: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
        let root = txt_input.trim().to_owned();
        txt_input.clear();
        // get progression type
        while (txt_input.trim() != "a") & (txt_input.trim() != "g") {
            txt_input.clear();
            print!("Arithmetic or geometric progrssion? (a/g): ");
            stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut txt_input)
                .expect("Problem reading input.");
            println!("{}", txt_input.trim());
        }
        let progression = if txt_input.trim() == "a" {
            ProgressionType::Arithmetic
        } else {
            ProgressionType::Geometric
        };
        txt_input.clear();
        // get starting number of particles
        print!("Starting number of particles: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
        let n_start: usize = txt_input.trim().parse().unwrap();
        txt_input.clear();
        // get step
        print!("Step: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
        let step: usize = txt_input.trim().parse().unwrap();
        txt_input.clear();
        // get number of iterations
        print!("Number of iterations: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt_input)
            .expect("Problem reading input.");
        println!("{}", txt_input.trim());
        let n_iter: usize = txt_input.trim().parse().unwrap();
        txt_input.clear();

        // Get data, process it, save results
        let timers = get_scaling_data(root, n_start, step, n_iter, progression);
        compile_scaling_data(&timers);
    }
    println!("Finished! All data is ready for use.")
}
