//! I/O handling
//!
//! This module contains all functions used to handle inputs and
//! outputs.

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::structures::{
    FiniteDiscreteRV, ProgressionType, SummarizedVariable, TimerReport, TimerSV, N_TALLIED_DATA,
};

// =======
// Reading

/// Builds an organized structure from a tallies report saved by the
/// `fastiron` binary.
///
/// This function is a general reading function,
/// not all the data will necessarly be used.
///
/// Any change done to the tallied data / its representation will
/// demand an update of this function.
pub fn read_tallies(file_name: &str) -> [FiniteDiscreteRV; N_TALLIED_DATA] {
    let file = File::open(file_name).unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut values: [Vec<f64>; N_TALLIED_DATA] = [
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
        Vec::with_capacity(100),
    ];
    // for each line
    for result in reader.records() {
        let mut record = result.unwrap();
        record.trim();
        // for each column
        (0..N_TALLIED_DATA).for_each(|idx| {
            let val = record.get(idx).unwrap();
            values[idx].push(val.parse().unwrap())
        })
    }
    // convert value vectors to our structure
    values.map(|val| FiniteDiscreteRV::new(&val))
}

/// Builds an organized structure from a timers report saved by the
/// `fastiron` binary.
///
/// This function is a general reading function,
/// not all the data will necessarly be used.
///
/// Any change done to the timers data / its representation will
/// demand an update of this function.
pub fn read_timers(file_name: &str) -> TimerReport {
    let mut res = [SummarizedVariable::default(); 6];
    let file = File::open(file_name).unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    // for each line
    for (timer_idx, result) in reader.records().enumerate() {
        let mut record = result.unwrap();
        record.trim();
        // lmao
        res[timer_idx].lowest = record.get(2).unwrap().parse().unwrap();
        res[timer_idx].mean = record.get(3).unwrap().parse().unwrap();
        res[timer_idx].highest = record.get(4).unwrap().parse().unwrap();
        res[timer_idx].total = record.get(5).unwrap().parse().unwrap();
    }

    TimerReport { timers_data: res }
}

/// Builds an organized structure from a (formatted) list of timers reports.
///
/// Any change done to the timers data / its representation will
/// demand an update of this function.
pub fn get_scaling_data(
    root: String,
    n_start: usize,
    step: usize,
    n_iter: usize,
    prog_type: ProgressionType,
) -> Vec<(TimerReport, usize)> {
    let n_particles = (0..n_iter).map(|idx| match prog_type {
        ProgressionType::Arithmetic => n_start + idx * step,
        ProgressionType::Geometric => n_start * step.pow(idx as u32),
    });
    n_particles
        .map(|n_particle| {
            let filename = format!("{}{}.csv", root, n_particle);
            (read_timers(&filename), n_particle)
        })
        .collect()
}

// =======
// Writing

/// Save the results of the comparison in a formatted markdown table.
///
/// Any change done to the timers data / its representation will
/// demand an update of this function.
pub fn save_percents(percents: &[f64]) {
    // Write the result in a Markdown table; maybe we can generate an entire report?
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("percents.md")
        .unwrap();
    writeln!(file, "| Section              | Percent Change |").unwrap();
    writeln!(file, "|----------------------|----------------|").unwrap();
    writeln!(file, "| Total execution time | {:>13.1}% |", percents[0]).unwrap();
    writeln!(file, "| PopulationControl    | {:>13.1}% |", percents[1]).unwrap();
    writeln!(file, "| CycleTracking        | {:>13.1}% |", percents[2]).unwrap();
    writeln!(file, "| CycleSync            | {:>13.1}% |", percents[3]).unwrap();
}

/// Save the results of the correlation study.
///
/// The plotted matrix will have this structure:
///
/// \* | Absorb | Scatter | Fission | Collision | Census | NumSeg
/// ---|--------|---------|---------|-----------|--------|--------
/// CycleTracking | `c0` | `c1` | `c2` | `c3` | `c4` | `c5`
///
pub fn save_tracking_results(tracking_res: &[f64]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tracking.dat")
        .unwrap();
    writeln!(file, ",Absorb,Scatter,Fission,Collision,Census,NumSeg").unwrap();
    // write correlation coeffs
    writeln!(
        file,
        "CycleTracking, {:.5}, {:.5}, {:.5}, {:.5}, {:.5}, {:.5}",
        tracking_res[0],
        tracking_res[1],
        tracking_res[2],
        tracking_res[3],
        tracking_res[4],
        tracking_res[5],
    )
    .unwrap();
    // padding values for it to be considered a matrix
    writeln!(file, "Dummy, 0, 0, 0, 0, 0, 0").unwrap();
}

/// Save the results of the correlation study.
///
/// The plotted matrix will have this structure:
///
/// \* | Rr | Split
/// ---|----|-------
/// PopulationControl | `c0` | `c1`
/// CycleSync         | `c2` | `c3`
///
pub fn save_popsync_results(popsync_res: &[f64]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("popsync.dat")
        .unwrap();
    writeln!(file, ",Rr,Split").unwrap();
    writeln!(
        file,
        "CycleSync, {:.5}, {:.5}",
        popsync_res[1], popsync_res[2]
    )
    .unwrap();
    writeln!(
        file,
        "PopulationControl, {:.5}, {:.5}",
        popsync_res[4], popsync_res[5]
    )
    .unwrap();
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
        "n_particles,PopulationControlAvg,CycleTrackingAvg,CycleSyncAvg"
    )
    .unwrap();
    timer_data.iter().for_each(|(report, n_particles)| {
        writeln!(
            file,
            "{},{},{},{}",
            n_particles,
            report[TimerSV::PopulationControl].mean,
            report[TimerSV::CycleTracking].mean,
            report[TimerSV::CycleSync].mean,
        )
        .unwrap();
    });
}
