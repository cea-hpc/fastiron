//! Code used for built-in timers & execution speed gauging
//!
//!

use std::{
    fmt::Display,
    fs::OpenOptions,
    io::Write,
    time::{Duration, Instant},
};

use crate::constants::sim::N_TIMERS;

/// Enum used to identify sections and their corresponding
/// timers.
#[derive(Debug)]
pub enum Section {
    /// Full execution time.
    Main = 0,
    /// `cycle_init()` execution time.
    PopulationControl,
    /// `cycle_tracking()` execution time.
    CycleTracking,
    /// Processing phase of `cycle_tracking()` execution time.
    CycleTrackingProcess,
    /// Sorting phase of `cycle_tracking()` execution time.
    CycleTrackingSort,
    /// `cycle_sync()` execution time.
    CycleSync,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Section::Main => write!(f, "Section::Main                 "),
            Section::PopulationControl => write!(f, "Section::PopulationControl    "),
            Section::CycleTracking => write!(f, "Section::CycleTracking        "),
            Section::CycleTrackingProcess => write!(f, "Section::CycleTrackingProcess "),
            Section::CycleTrackingSort => write!(f, "Section::CycleTrackingSort    "),
            Section::CycleSync => write!(f, "Section::CycleSync            "),
        }
    }
}

/// Structure used to represent a single timer.
#[derive(Debug)]
pub struct MCFastTimer {
    /// Clock value at the start of the timer.
    pub start_clock: Instant,
    /// Clock value at the start of the timer.
    pub end_clock: Instant,
    /// Value of the last duration in microseconds.
    pub last_cycle_clock: Duration,
    /// Value of the total duration in microseconds.
    pub cumulative_clock: Duration,
    /// Number of call to the timer i.e. number of measurement taken.
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

/// Structure used as a container for the 6 timers used through
/// the simulation for performance testing.
///
/// See [Section] for more information about the different timers.
#[derive(Debug)]
pub struct MCFastTimerContainer {
    /// Timer container.
    pub timers: [MCFastTimer; N_TIMERS],
    /// Current number of value used for average computation. Used to
    /// compute the new average after recording an additional value.
    pub n_avg: u32,
    /// Average duration of each timer.
    pub avgs: [Duration; N_TIMERS],
    /// Longest duration of each timer.
    pub maxs: [Duration; N_TIMERS],
    /// Shortest duration of each timer.
    pub mins: [Duration; N_TIMERS],
    /// Total duration of each timer. This is the value that should be compared to
    /// Quicksilver's cumulative report despite what its header says.
    pub tots: [Duration; N_TIMERS],
}

impl MCFastTimerContainer {
    /// Prints the cumulative report at the end of execution. The values of the  
    /// total column should be compared to Quicksilver's cumulative report despite
    /// what its header says.
    ///
    /// Here is an example output:
    ///
    /// ```shell
    /// Timer Name                     | Total # of calls | Shortest cycle (µs) | Average per cycle (µs) | Longest cycle (µs) | Total in section (µs) | Efficiency rating (%)
    /// Section::Main                  |                1 |     1.027135e7      |        1.027135e7      |    1.027135e7      |       1.027135e7      |             100.0
    /// Section::PopulationControl     |               10 |     9.609000e3      |        1.179200e4      |    1.645900e4      |       1.179200e5      |              71.6
    /// Section::CycleTracking         |               10 |     9.942510e5      |        1.014646e6      |    1.024305e6      |       1.014647e7      |              99.1
    /// Section::CycleTrackingProcess  |               10 |     9.942500e5      |        1.014646e6      |    1.024305e6      |       1.014646e7      |              99.1
    /// Section::CycleTrackingSort     |               10 |     0.000000e0      |        0.000000e0      |    0.000000e0      |       2.000000e0      |              38.5
    /// Section::CycleSync             |               11 |     2.540000e2      |        6.280000e2      |    8.360000e2      |       6.283000e3      |              75.1
    /// Figure of merit: 2.203e6 [segments / cycle tracking time]
    /// ```
    ///
    /// [`Section::PopulationControl`] and [`Section::CycleSync`] do not really have an
    /// equivalent in Quicksilver, the comparable values are those of [`Section::Main`]
    /// and [`Section::CycleTracking`] as well as the figure of merit.
    pub fn cumulative_report(&self, num_segments: u64, csv: bool) {
        // Print header
        println!("[Timer Report]");
        println!(
            "Timer Name                     | {:16} | {:19} | {:22} | {:18} | {:21} | {:21}",
            "Total # of calls",
            "Shortest cycle (µs)",
            "Average per cycle (µs)",
            "Longest cycle (µs)",
            "Total in section (µs)",
            "Efficiency rating (%)",
        );
        if csv {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("timers_report.csv")
                .unwrap();
            writeln!(
                file,
                "Timer Name;#calls;Shortest(µs);Average(µs);Longest(µs);Total(µs);Efficiency(%)",
            )
            .unwrap();
        }
        // print data
        self.timers
            .iter()
            .enumerate()
            .for_each(|(timer_idx, timer)| {
                let section = match timer_idx {
                    0 => Section::Main,
                    1 => Section::PopulationControl,
                    2 => Section::CycleTracking,
                    3 => Section::CycleTrackingProcess,
                    4 => Section::CycleTrackingSort,
                    5 => Section::CycleSync,
                    _ => unreachable!(),
                };
                println!(
                    "{} | {:>16} | {:>14.6e}      | {:>17.6e}      | {:>13.6e}      | {:>16.6e}      | {:>17.1}",
                    section,
                    timer.num_calls,
                    self.mins[timer_idx].as_micros(),
                    self.avgs[timer_idx].as_micros(),
                    self.maxs[timer_idx].as_micros(),
                    self.tots[timer_idx].as_micros(),
                    (100.0 * self.avgs[timer_idx].as_secs_f64())
                        / (self.maxs[timer_idx].as_secs_f64() + 1.0e-80),
                );
                if csv {
                    let mut file = OpenOptions::new()
                        .append(true)
                        .open("timers_report.csv")
                        .unwrap();
                    writeln!(file, "{};{};{:e};{:e};{:e};{:e};{:.1}", 
                        section,
                        timer.num_calls,
                        self.mins[timer_idx].as_micros(),
                        self.avgs[timer_idx].as_micros(),
                        self.maxs[timer_idx].as_micros(),
                        self.tots[timer_idx].as_micros(),
                        (100.0 * self.avgs[timer_idx].as_secs_f64())
                            / (self.maxs[timer_idx].as_secs_f64() + 1.0e-80)
                    ).unwrap();
                };
            });
        println!(
            "Figure of merit: {:>.3e} [segments / cycle tracking time]",
            (num_segments as f64) / (self.tots[Section::CycleTracking as usize].as_secs_f64())
        );
    }

    /// Update statistics and clear the timers for the next cycle.
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
                self.tots[timer_idx] += timer.last_cycle_clock;
                if self.mins[timer_idx] > timer.last_cycle_clock {
                    self.mins[timer_idx] = timer.last_cycle_clock;
                }
                if self.maxs[timer_idx] < timer.last_cycle_clock {
                    // cant be a max and a min
                    self.maxs[timer_idx] = timer.last_cycle_clock;
                }
                // new_avg = old_avg * N-1/N + new_val/N
                self.avgs[timer_idx] =
                    (self.avgs[timer_idx] * (self.n_avg - 1) + timer.last_cycle_clock) / self.n_avg;

                // clear timers
                timer.last_cycle_clock = Duration::ZERO;
            });
    }

    /// Update the statistics of the main timer.
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

/// Start the specified timer.
pub fn start(timer_container: &mut MCFastTimerContainer, section: Section) {
    let index = section as usize;
    timer_container.timers[index].start_clock = Instant::now();
}

/// Stop the specified timer and record internally the duration sicne start.
pub fn stop(timer_container: &mut MCFastTimerContainer, section: Section) {
    let index = section as usize;
    timer_container.timers[index].end_clock = Instant::now();
    timer_container.timers[index].last_cycle_clock += timer_container.timers[index]
        .end_clock
        .duration_since(timer_container.timers[index].start_clock);
    timer_container.timers[index].cumulative_clock += timer_container.timers[index]
        .end_clock
        .duration_since(timer_container.timers[index].start_clock);
    timer_container.timers[index].num_calls += 1;
}

/// Returns the duration of the last cycle of the specified timer.
pub fn get_last_cycle(timer_container: &mut MCFastTimerContainer, section: Section) -> f64 {
    let index = section as usize;
    timer_container.timers[index].last_cycle_clock.as_secs_f64()
}
