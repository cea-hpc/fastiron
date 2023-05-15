//! Data computation & analysis
//!
//! This module contains all functions used for data processing.

use crate::structures::{correlation, FiniteDiscreteRV, TalliedData, TimerReport, TimerSV};

/// Pairs of tallied data to be correlated.
///
/// This constant is used for the `PopulationControl`/`CycleSync` section
/// of the correlation study.
pub const POPSYNC_CORRELATIONS: [(TalliedData, TalliedData); 6] = [
    (TalliedData::Source, TalliedData::PopulationControl),
    (TalliedData::Source, TalliedData::CycleSync),
    (TalliedData::Rr, TalliedData::PopulationControl),
    (TalliedData::Rr, TalliedData::CycleSync),
    (TalliedData::Split, TalliedData::PopulationControl),
    (TalliedData::Split, TalliedData::CycleSync),
];

/// Pairs of tallied data to be correlated.
///
/// This constant is used for the `CycleTracking` section
/// of the correlation study.
pub const TRACKING_CORRELATIONS: [(TalliedData, TalliedData); 6] = [
    (TalliedData::Absorb, TalliedData::CycleTracking),
    (TalliedData::Scatter, TalliedData::CycleTracking),
    (TalliedData::Fission, TalliedData::CycleTracking),
    (TalliedData::Collision, TalliedData::CycleTracking),
    (TalliedData::Census, TalliedData::CycleTracking),
    (TalliedData::NumSeg, TalliedData::CycleTracking),
];

/// Computes the relative change between two timers reports.
///
/// The actual relative change is multiplied by 100 to have percentages
/// as a result.
pub fn compare(old: TimerReport, new: TimerReport) -> [f64; 4] {
    let relative_change =
        |section: TimerSV| (new[section].mean - old[section].mean) / old[section].mean;

    let exec_time = relative_change(TimerSV::Main) * 100.0;
    let pop_control = relative_change(TimerSV::PopulationControl) * 100.0;
    let tracking = relative_change(TimerSV::CycleTracking) * 100.0;
    let sync = relative_change(TimerSV::CycleSync) * 100.0;

    [exec_time, pop_control, tracking, sync]
}

/// Computes correlation coefficient for the correlation study.
///
/// This function computes coefficient for the `PopulationControl`/`CycleSync`
/// section of the correlation study.
pub fn build_popsync_results(tallies_data: &[FiniteDiscreteRV]) -> Vec<f64> {
    // The table is something like this
    //
    //                   | Source | Rr | Split
    // PopulationControl | ...
    // CycleSync         | ...
    //
    // gnuplot has the Y axis upside down, hence the vector:
    vec![
        correlation(
            &tallies_data[TalliedData::Source as usize],
            &tallies_data[TalliedData::CycleSync as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Rr as usize],
            &tallies_data[TalliedData::CycleSync as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Split as usize],
            &tallies_data[TalliedData::CycleSync as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Source as usize],
            &tallies_data[TalliedData::PopulationControl as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Rr as usize],
            &tallies_data[TalliedData::PopulationControl as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Split as usize],
            &tallies_data[TalliedData::PopulationControl as usize],
        ),
    ]
}

/// Computes correlation coefficient for the correlation study.
///
/// This function computes coefficient for the `CycleTracking` section
/// of the correlation study.
pub fn build_tracking_results(tallies_data: &[FiniteDiscreteRV]) -> Vec<f64> {
    vec![
        correlation(
            &tallies_data[TalliedData::Absorb as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Scatter as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Fission as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Collision as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::Census as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
        correlation(
            &tallies_data[TalliedData::NumSeg as usize],
            &tallies_data[TalliedData::CycleTracking as usize],
        ),
    ]
}
