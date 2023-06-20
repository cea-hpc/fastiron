use super::raw::{TalliedData, TimerReport, N_TIMERS};

//~~~~~~~~~~~~~~~~~
// Comparison data
//~~~~~~~~~~~~~~~~~

pub struct ComparisonResults {
    pub old: TimerReport,
    pub new: TimerReport,
    pub percents: [f64; N_TIMERS],
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
