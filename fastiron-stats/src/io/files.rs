//! I/O handling
//!
//! This module contains all functions used to handle inputs and
//! outputs.

use std::{fs::OpenOptions, io::Write};

use crate::structures::raw::{TimerReport, TimerSV};

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
