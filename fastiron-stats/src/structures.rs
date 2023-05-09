use std::{iter::zip, ops::Index};

pub const N_TALLIED_DATA: usize = 17;

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

#[derive(Debug, Clone, Copy)]
pub enum TimerSV {
    Main = 0,
    PopulationControl = 1,
    CycleTracking = 2,
    CycleTrackingKernel = 3,
    CycleTrackingComm = 4,
    CycleSync = 5,
}

#[derive(Debug, Clone, Copy)]
pub enum ProgressionType {
    Arithmetic,
    Geometric,
}

#[derive(Debug)]
pub struct FiniteDiscreteRV {
    pub values: Vec<f64>,
    pub mean: f64,
    pub variance: f64,
}

impl FiniteDiscreteRV {
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

    pub fn n_val(&self) -> usize {
        self.values.len()
    }
}

pub fn covariance(x: &FiniteDiscreteRV, y: &FiniteDiscreteRV) -> f64 {
    assert_eq!(x.n_val(), y.n_val());
    let iter = zip(x.values.iter(), y.values.iter());
    let mut cov = iter.map(|(xi, yi)| (xi - x.mean) * (yi - y.mean)).sum();
    cov /= x.n_val() as f64;
    cov
}

pub fn correlation(x: &FiniteDiscreteRV, y: &FiniteDiscreteRV) -> f64 {
    if (x.variance == 0.0) | (y.variance == 0.0) {
        // 0 means variables are independent
        // this may be technically false but it allows for generic computations
        return 0.0;
    }
    let cov = covariance(x, y);
    cov / (x.variance * y.variance).sqrt()
}

#[derive(Default, Clone, Copy, Debug)]
pub struct SummarizedVariable {
    pub mean: f64,
    pub lowest: f64,
    pub highest: f64,
    pub total: f64,
}

pub struct TimerReport {
    pub timers_data: [SummarizedVariable; 6],
}

impl Index<TimerSV> for TimerReport {
    type Output = SummarizedVariable;

    fn index(&self, timer: TimerSV) -> &Self::Output {
        &self.timers_data[timer as usize]
    }
}
