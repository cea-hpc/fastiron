use std::{fs::OpenOptions, io::Write};

use super::raw::{TalliedData, TimerReport, TimerSV, N_TIMERS};

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

pub const CORRELATIONS: ([TalliedData; 11], [TalliedData; 4]) = (
    [
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
    ],
    [
        TalliedData::Cycle,
        TalliedData::PopulationControl,
        TalliedData::CycleTracking,
        TalliedData::CycleSync,
    ],
);
