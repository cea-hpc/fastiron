use std::{fs::OpenOptions, io::Write};

use super::raw::{correlation, TalliedData, TalliesReport, TimerReport, TimerSV, N_TIMERS};

//~~~~~~~~~~~~~~~~~
// Comparison data
//~~~~~~~~~~~~~~~~~

pub struct ComparisonResults {
    pub old: TimerReport,
    pub new: TimerReport,
    pub percents: [f64; N_TIMERS],
}

impl ComparisonResults {
    pub fn save(&self) {
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
            self.old[TimerSV::Main].mean,
            self.new[TimerSV::Main].mean,
            self.percents[0]
        )
        .unwrap();
        writeln!(
            file,
            "{},{},{},{}",
            TimerSV::PopulationControl,
            self.old[TimerSV::PopulationControl].mean,
            self.new[TimerSV::PopulationControl].mean,
            self.percents[1]
        )
        .unwrap();
        writeln!(
            file,
            "{},{},{},{}",
            TimerSV::CycleTracking,
            self.old[TimerSV::CycleTracking].mean,
            self.new[TimerSV::CycleTracking].mean,
            self.percents[2]
        )
        .unwrap();
        writeln!(
            file,
            "{},{},{},{}",
            TimerSV::CycleTrackingProcess,
            self.old[TimerSV::CycleTrackingProcess].mean,
            self.new[TimerSV::CycleTrackingProcess].mean,
            self.percents[3]
        )
        .unwrap();
        writeln!(
            file,
            "{},{},{},{}",
            TimerSV::CycleTrackingSort,
            self.old[TimerSV::CycleTrackingSort].mean,
            self.new[TimerSV::CycleTrackingSort].mean,
            self.percents[4]
        )
        .unwrap();
        writeln!(
            file,
            "{},{},{},{}",
            TimerSV::CycleSync,
            self.old[TimerSV::CycleSync].mean,
            self.new[TimerSV::CycleSync].mean,
            self.percents[5]
        )
        .unwrap();
    }

    pub fn plot(&self) {
        todo!()
    }
}

impl From<(TimerReport, TimerReport)> for ComparisonResults {
    fn from((old, new): (TimerReport, TimerReport)) -> Self {
        let relative_change =
            |section: TimerSV| (new[section].mean - old[section].mean) / old[section].mean;

        let percents = [
            TimerSV::Main,
            TimerSV::PopulationControl,
            TimerSV::CycleTracking,
            TimerSV::CycleTrackingProcess,
            TimerSV::CycleTrackingSort,
            TimerSV::CycleSync,
        ]
        .map(|section| relative_change(section) * 100.0);

        Self { old, new, percents }
    }
}

//~~~~~~~~~~~~~~~~~~
// Correlation data
//~~~~~~~~~~~~~~~~~~

pub const CORRELATION_COLS: [TalliedData; 11] = [
    TalliedData::Start,
    TalliedData::Rr,
    TalliedData::Split,
    TalliedData::Absorb,
    TalliedData::Scatter,
    TalliedData::Fission,
    TalliedData::Produce,
    TalliedData::Collision,
    TalliedData::Escape,
    TalliedData::Census,
    TalliedData::NumSeg,
];

pub const CORRELATION_ROWS: [TalliedData; 4] = [
    TalliedData::Cycle,
    TalliedData::PopulationControl,
    TalliedData::CycleTracking,
    TalliedData::CycleSync,
];

pub struct CorrelationResults {
    pub corr_data: [f64; 11 * 4],
}

impl CorrelationResults {
    pub fn save(&self) {
        todo!()
    }

    pub fn plot(&self) {
        todo!()
    }
}

impl From<TalliesReport> for CorrelationResults {
    fn from(report: TalliesReport) -> Self {
        // compute correlations
        let table = CORRELATION_ROWS.map(|tallied_data| {
            CORRELATION_COLS
                .map(|tallied_event| correlation(&report[tallied_data], &report[tallied_event]))
        });

        // a little black magic to flatten the array
        let flat_table: &[f64; 11 * 4] = unsafe { std::mem::transmute(&table) };

        Self {
            corr_data: *flat_table,
        }
    }
}
