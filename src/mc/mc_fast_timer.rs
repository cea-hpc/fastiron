use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::{
    constants::{sim::N_TIMERS, CustomFloat},
    montecarlo::MonteCarlo,
};

/// Enum used to identify sections and their corresponding
/// timers.
#[derive(Debug)]
pub enum Section {
    Main = 0,
    CycleInit,
    CycleTracking,
    CycleTrackingKernel,
    CycleTrackingComm,
    CycleFinalize,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Section::Main => write!(f, "Section::Main                 "),
            Section::CycleInit => write!(f, "Section::CycleInit            "),
            Section::CycleTracking => write!(f, "Section::CycleTracking        "),
            Section::CycleTrackingKernel => write!(f, "Section::CycleTrackingKernel  "),
            Section::CycleTrackingComm => write!(f, "Section::CycleTrackingComm    "),
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
#[derive(Debug)]
pub struct MCFastTimerContainer {
    pub timers: [MCFastTimer; N_TIMERS],
    pub n_avg: u32,
    pub avgs: [Duration; N_TIMERS],
    pub maxs: [Duration; N_TIMERS],
    pub mins: [Duration; N_TIMERS],
    pub tots: [Duration; N_TIMERS],
}

impl MCFastTimerContainer {
    pub fn cumulative_report(&self, num_segments: u64) {
        // Print header
        println!("[Timer Report]");
        println!("Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)");
        self.timers
            .iter()
            .enumerate()
            .for_each(|(timer_idx, timer)| {
                let section = match timer_idx {
                    0 => Section::Main,
                    1 => Section::CycleInit,
                    2 => Section::CycleTracking,
                    3 => Section::CycleTrackingKernel,
                    4 => Section::CycleTrackingComm,
                    5 => Section::CycleFinalize,
                    _ => unreachable!(),
                };
                println!(
                    "{}   | {:>21}    {:>16e}    {:>22e}     {:>18e}    {:>21e}    {:>22.1}",
                    section,
                    timer.num_calls,
                    self.mins[timer_idx].as_micros(),
                    self.avgs[timer_idx].as_micros(),
                    self.maxs[timer_idx].as_micros(),
                    self.tots[timer_idx].as_micros(),
                    (100.0 * self.avgs[timer_idx].as_secs_f64())
                        / (self.maxs[timer_idx].as_secs_f64() + 1.0e-80),
                );
            });
        println!(
            "Figure of merit: {:>.3e} [segments / cycle tracking time]",
            (num_segments as f64) / (self.tots[Section::CycleTracking as usize].as_secs_f64())
        );
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
                    4 => Section::CycleTrackingComm,
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
        self.n_avg += 1;
        self.timers
            .iter_mut()
            .enumerate()
            .for_each(|(timer_idx, timer)| {
                if timer_idx == Section::Main as usize {
                    return;
                }
                // update internal values for report
                self.tots[timer_idx] += timer.end_clock.duration_since(timer.start_clock);
                if self.mins[timer_idx] > timer.end_clock.duration_since(timer.start_clock) {
                    self.mins[timer_idx] = timer.end_clock.duration_since(timer.start_clock);
                } else if self.maxs[timer_idx] < timer.end_clock.duration_since(timer.start_clock) {
                    // cant be a max and a min
                    self.maxs[timer_idx] = timer.end_clock.duration_since(timer.start_clock);
                }
                // new_avg = old_avg * N-1/N + new_val/N
                self.avgs[timer_idx] = (self.avgs[timer_idx] * (self.n_avg - 1)
                    + timer.end_clock.duration_since(timer.start_clock))
                    / self.n_avg;

                // clear timers
                timer.last_cycle_clock = 0;
            });
    }

    pub fn update_main_stats(&mut self) {
        let idx = Section::Main as usize;
        let duration = self.timers[idx]
            .end_clock
            .duration_since(self.timers[idx].start_clock);
        self.avgs[idx] = duration;
        self.mins[idx] = duration;
        self.maxs[idx] = duration;
        self.tots[idx] = duration;
    }
}

impl Default for MCFastTimerContainer {
    fn default() -> Self {
        Self {
            timers: Default::default(),
            avgs: [Duration::ZERO; N_TIMERS],
            n_avg: 0,
            maxs: [Duration::ZERO; N_TIMERS],
            mins: [Duration::MAX; N_TIMERS],
            tots: [Duration::ZERO; N_TIMERS],
        }
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
