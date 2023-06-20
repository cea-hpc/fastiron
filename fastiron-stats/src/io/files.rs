//! I/O handling
//!
//! This module contains all functions used to handle inputs and
//! outputs.

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::structures::raw::{TimerReport, TimerSV};

/// Builds an organized structure from a (formatted) list of timers reports.
///
/// Any change done to the timers data / its representation will
/// demand an update of this function.
pub fn get_scaling_data(
    root: String,
    n_start: usize,
    step: usize,
    n_iter: usize,
) -> Vec<(TimerReport, usize)> {
    let n_threads = (0..n_iter).map(|idx| n_start * step.pow(idx as u32));
    n_threads
        .map(|n_thread| {
            let filename = format!("{}{}.csv", root, n_thread);
            (TimerReport::from(File::open(filename).unwrap()), n_thread)
        })
        .collect()
}

/// Save the results of the scaling study.
///
/// The produced file technically fits the csv format. For
/// consistency, it is saved as a dat file.
///
/// The file contains four columns, each line corresponds to one simulation:
///
/// - `n_particles`: target number of particles.
/// - `PopulationControlAvg`, `CycleTrackingAvg`, `CycleSyncAvg`:
///   Average time spent in the given section.
///
pub fn compile_scaling_data(timer_data: &[(TimerReport, usize)]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("scaling.dat")
        .unwrap();
    // we assume correct ordering of the summarized variables
    // i.e. lowest number of particle to highest
    writeln!(
        file,
        "n_threads,PopulationControlAvg,CycleTrackingAvg,CycleSyncAvg"
    )
    .unwrap();
    timer_data.iter().for_each(|(report, n_threads)| {
        writeln!(
            file,
            "{},{},{},{}",
            n_threads,
            report[TimerSV::PopulationControl].mean,
            report[TimerSV::CycleTracking].mean,
            report[TimerSV::CycleSync].mean,
        )
        .unwrap();
    });
}
