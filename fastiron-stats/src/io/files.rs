//! I/O handling
//!
//! This module contains all functions used to handle inputs and
//! outputs.

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::structures::{
    FiniteDiscreteRV, SummarizedVariable, TimerReport, TimerSV, N_TALLIED_DATA, N_TIMERS,
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
    let mut values: [Vec<f64>; N_TALLIED_DATA] = Default::default();
    values.iter_mut().for_each(|v| v.reserve(100));
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
    let mut res = [SummarizedVariable::default(); N_TIMERS];
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
) -> Vec<(TimerReport, usize)> {
    let n_threads = (0..n_iter).map(|idx| n_start * step.pow(idx as u32));
    n_threads
        .map(|n_thread| {
            let filename = format!("{}{}.csv", root, n_thread);
            (read_timers(&filename), n_thread)
        })
        .collect()
}

// =======
// Writing

/// Save the results of the comparison.
///
/// Any change done to the timers data / its representation will
/// demand an update of this function.
pub fn save_percents(old: TimerReport, new: TimerReport, percents: &[f64]) {
    assert_eq!(percents.len(), N_TIMERS);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("comparison.csv")
        .unwrap();
    writeln!(file, "section,old,new,change").unwrap();
    writeln!(
        file,
        "{},{},{},{}",
        TimerSV::Main,
        old[TimerSV::Main].mean,
        new[TimerSV::Main].mean,
        percents[0]
    )
    .unwrap();
    writeln!(
        file,
        "{},{},{},{}",
        TimerSV::PopulationControl,
        old[TimerSV::PopulationControl].mean,
        new[TimerSV::PopulationControl].mean,
        percents[1]
    )
    .unwrap();
    writeln!(
        file,
        "{},{},{},{}",
        TimerSV::CycleTracking,
        old[TimerSV::CycleTracking].mean,
        new[TimerSV::CycleTracking].mean,
        percents[2]
    )
    .unwrap();
    writeln!(
        file,
        "{},{},{},{}",
        TimerSV::CycleTrackingProcess,
        old[TimerSV::CycleTrackingProcess].mean,
        new[TimerSV::CycleTrackingProcess].mean,
        percents[3]
    )
    .unwrap();
    writeln!(
        file,
        "{},{},{},{}",
        TimerSV::CycleTrackingSort,
        old[TimerSV::CycleTrackingSort].mean,
        new[TimerSV::CycleTrackingSort].mean,
        percents[4]
    )
    .unwrap();
    writeln!(
        file,
        "{},{},{},{}",
        TimerSV::CycleSync,
        old[TimerSV::CycleSync].mean,
        new[TimerSV::CycleSync].mean,
        percents[5]
    )
    .unwrap();
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
