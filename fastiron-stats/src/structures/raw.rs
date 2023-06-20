//! Modelling code
//!
//! This module contains all structures used to model the data and
//! structure the computations.

use std::{fmt::Display, fs::File, iter::zip, ops::Index};

//~~~~~~~~~~~~~~
// Tallies data
//~~~~~~~~~~~~~~

/// Number of fields presented in the tallies report produced
/// by `fastiron`.
pub const N_TALLIED_DATA: usize = 17;

/// Enum used to represent & map tallied data and their indexes.
#[derive(Debug)]
pub enum TalliedData {
    Cycle = 0,
    Start = 1,
    Source = 2,
    Rr = 3,
    Split = 4,
    Absorb = 5,
    Scatter = 6,
    Fission = 7,
    Produce = 8,
    Collision = 9,
    Escape = 10,
    Census = 11,
    NumSeg = 12,
    ScalarFlux = 13,
    PopulationControl = 14,
    CycleTracking = 15,
    CycleSync = 16,
}

/// Structure used to model finite discrete random variables.
///
/// This structure is not meant to be modified. It should be initialized with all
/// values using the provided constructor.
#[derive(Debug)]
pub struct FiniteDiscreteRV {
    /// Values taken by the random variables.
    pub values: Vec<f64>,
    /// Associated mean.
    pub mean: f64,
    /// Associated variance.
    pub variance: f64,
}

impl FiniteDiscreteRV {
    /// Constructor. Takes a slice as values and computes key associated values
    /// before returning the object.
    pub fn new(values: &[f64]) -> Self {
        let n_val = values.len() as f64;
        let val = values.to_vec();
        let mut mean = val.iter().sum();
        mean /= n_val;
        let mut var = val.iter().map(|xi| (xi - mean) * (xi - mean)).sum();
        var /= n_val;

        Self {
            values: val,
            mean,
            variance: var,
        }
    }

    /// Returns the number of values taken by the (discrete, finite) random variable
    pub fn n_val(&self) -> usize {
        self.values.len()
    }
}

/// Returns the covariance of two given [FiniteDiscreteRV].
pub fn covariance(x: &FiniteDiscreteRV, y: &FiniteDiscreteRV) -> f64 {
    assert_eq!(x.n_val(), y.n_val());
    let iter = zip(x.values.iter(), y.values.iter());
    let mut cov = iter.map(|(xi, yi)| (xi - x.mean) * (yi - y.mean)).sum();
    cov /= x.n_val() as f64;
    cov
}

/// Returns the correlation coefficient of two given [FiniteDiscreteRV].
///
/// The function checks if `x` and `y` have non-zero variance. If this is the case,
/// 0 is returned. It means variables are independent. While this may be technically
/// false, it allows for generic computations
pub fn correlation(x: &FiniteDiscreteRV, y: &FiniteDiscreteRV) -> f64 {
    if (x.variance == 0.0) | (y.variance == 0.0) {
        //
        return 0.0;
    }
    let cov = covariance(x, y);
    cov / (x.variance * y.variance).sqrt()
}

pub struct TalliesReport {
    pub tallies_data: [FiniteDiscreteRV; N_TALLIED_DATA],
}

impl From<File> for TalliesReport {
    fn from(file: File) -> Self {
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
        Self {
            tallies_data: values.map(|val| FiniteDiscreteRV::new(&val)),
        }
    }
}

impl Index<TalliedData> for TalliesReport {
    type Output = FiniteDiscreteRV;

    fn index(&self, tallied_data: TalliedData) -> &Self::Output {
        &self.tallies_data[tallied_data as usize]
    }
}

//~~~~~~~~~~~~
// Timer data
//~~~~~~~~~~~~

pub const N_TIMERS: usize = 6;

/// Enum used to represent & map timers breakdown and their indexes.
#[derive(Debug, Clone, Copy)]
pub enum TimerSV {
    Main = 0,
    PopulationControl = 1,
    CycleTracking = 2,
    CycleTrackingProcess = 3,
    CycleTrackingSort = 4,
    CycleSync = 5,
}

impl Display for TimerSV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TimerSV::Main => "Section::Main",
                TimerSV::PopulationControl => "Section::PopulationControl",
                TimerSV::CycleTracking => "Section::CycleTracking",
                TimerSV::CycleTrackingProcess => "Section::CycleTrackingProcess",
                TimerSV::CycleTrackingSort => "Section::CycleTrackingSort",
                TimerSV::CycleSync => "Section::CycleSync",
            }
        )
    }
}

/// Structure used to represent the summarized data of the timers.
#[derive(Default, Clone, Copy, Debug)]
pub struct SummarizedVariable {
    /// Average value taken by the timer.
    pub mean: f64,
    /// Lowest value taken by the timer.
    pub lowest: f64,
    /// Highest value taken by the timer.
    pub highest: f64,
    /// Sum of all value taken by the timer.
    pub total: f64,
}

/// Structure used to reprensent the entire timer report of a single simulation
pub struct TimerReport {
    /// Array of the section timers.
    pub timers_data: [SummarizedVariable; N_TIMERS],
}

impl From<File> for TimerReport {
    fn from(file: File) -> Self {
        let mut res = [SummarizedVariable::default(); N_TIMERS];
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

        Self { timers_data: res }
    }
}

impl Index<TimerSV> for TimerReport {
    type Output = SummarizedVariable;

    fn index(&self, timer: TimerSV) -> &Self::Output {
        &self.timers_data[timer as usize]
    }
}
