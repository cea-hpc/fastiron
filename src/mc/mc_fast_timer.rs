use std::{fmt::Display, time::Instant};

use crate::{constants::CustomFloat, montecarlo::MonteCarlo};

/// Enum used to identify sections and their corresponding
/// timers.
#[derive(Debug)]
pub enum Section {
    Main = 0,
    CycleInit,
    CycleTracking,
    CycleTrackingKernel,
    CycleTrackingMPI,
    CycleTrackingTestDone,
    CycleFinalize,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Section::Main => write!(f, "Section::Main                 "),
            Section::CycleInit => write!(f, "Section::CycleInit            "),
            Section::CycleTracking => write!(f, "Section::CycleTracking        "),
            Section::CycleTrackingKernel => write!(f, "Section::CycleTrackingKernel  "),
            Section::CycleTrackingMPI => write!(f, "Section::CycleTrackingMPI     "),
            Section::CycleTrackingTestDone => write!(f, "Section::CycleTrackingTestDone"),
            Section::CycleFinalize => write!(f, "Section::CycleFinalize        "),
        }
    }
}

/// Structure used to represent a single timer.
#[derive(Debug)]
pub struct MCFastTimer {
    pub start_clock: Instant,
    pub end_clock: Instant,
    pub last_cycle_clock: u128,
    pub cumulative_clock: u128,
    pub num_calls: u64,
}

impl Default for MCFastTimer {
    fn default() -> Self {
        Self {
            start_clock: Instant::now(),
            end_clock: Instant::now(),
            last_cycle_clock: Default::default(),
            cumulative_clock: Default::default(),
            num_calls: Default::default(),
        }
    }
}

/// Structure used as a container for the 7 timers used through
/// the simulation for performance testing.
#[derive(Debug, Default)]
pub struct MCFastTimerContainer {
    pub timers: [MCFastTimer; 7],
}

impl MCFastTimerContainer {
    pub fn cumulative_report(&self) {
        // TODO: COMPLETE
        // Print header
        println!("Timer Name                        Last cycle number of calls   Last cycle min (ms)    Last cycle avg (ms)    Last cycle max (ms)    Last cycle stddev (ms)    Last cycle efficiency rating");
        self.timers
            .iter()
            .enumerate()
            .for_each(|(timer_idx, timer)| {
                let section = match timer_idx {
                    0 => Section::Main,
                    1 => Section::CycleInit,
                    2 => Section::CycleTracking,
                    3 => Section::CycleTrackingKernel,
                    4 => Section::CycleTrackingMPI,
                    5 => Section::CycleTrackingTestDone,
                    6 => Section::CycleFinalize,
                    _ => unreachable!(),
                };
                println!(
                    "{}    {}    {}    {}    {}    {}    {}",
                    section, timer.num_calls, 0, 0, 0, 0, 0
                );
            });
    }

    pub fn last_cycle_report(&self) {
        // TODO: COMPLETE
        // Print header
        println!("Timer Name                        Last cycle number of calls   Last cycle min (ms)    Last cycle avg (ms)    Last cycle max (ms)    Last cycle stddev (ms)    Last cycle efficiency rating");
        self.timers
            .iter()
            .enumerate()
            .for_each(|(timer_idx, timer)| {
                let section = match timer_idx {
                    0 => Section::Main,
                    1 => Section::CycleInit,
                    2 => Section::CycleTracking,
                    3 => Section::CycleTrackingKernel,
                    4 => Section::CycleTrackingMPI,
                    5 => Section::CycleTrackingTestDone,
                    6 => Section::CycleFinalize,
                    _ => unreachable!(),
                };
                println!(
                    "{}    {}    {}    {}    {}    {}    {}",
                    section, timer.num_calls, 0, 0, 0, 0, 0
                );
            });
    }

    pub fn clear_last_cycle_timers(&mut self) {
        self.timers.iter_mut().for_each(|timer| {
            timer.last_cycle_clock = 0;
        });
    }
}

pub fn start<T: CustomFloat>(mcco: &mut MonteCarlo<T>, section: Section) {
    let index = section as usize;
    mcco.fast_timer.timers[index].start_clock = Instant::now();
}

pub fn stop<T: CustomFloat>(mcco: &mut MonteCarlo<T>, section: Section) {
    let index = section as usize;
    mcco.fast_timer.timers[index].end_clock = Instant::now();
    mcco.fast_timer.timers[index].last_cycle_clock += mcco.fast_timer.timers[index]
        .end_clock
        .duration_since(mcco.fast_timer.timers[index].start_clock)
        .as_micros();
    mcco.fast_timer.timers[index].cumulative_clock += mcco.fast_timer.timers[index]
        .end_clock
        .duration_since(mcco.fast_timer.timers[index].start_clock)
        .as_micros();
    mcco.fast_timer.timers[index].num_calls += 1;
}

pub fn get_last_cycle<T: CustomFloat>(mcco: &MonteCarlo<T>, section: Section) -> f64 {
    let index = section as usize;
    mcco.fast_timer.timers[index].last_cycle_clock as f64 / 1000000.0
}
