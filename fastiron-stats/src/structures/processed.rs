use super::raw::TalliedData;

//~~~~~~~~~~~~~~
// Tallies data
//~~~~~~~~~~~~~~

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

//~~~~~~~~~~~~~
// Timers data
//~~~~~~~~~~~~~
