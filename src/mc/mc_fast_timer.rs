use std::{cell::RefCell, rc::Rc, time::Instant};

use num::Float;

use crate::montecarlo::MonteCarlo;

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
    pub timers: [MCFastTimer; 7],
}

impl MCFastTimerContainer {
    /*
    pub fn start(&mut self, index: usize) {
        self.timers[index].start_clock = Instant::now();
    }
    */
    pub fn cumulative_report(&self) {
        todo!()
    }

    pub fn last_cycle_report(&self) {
        todo!()
    }

    pub fn clear_last_cycle_timers(&self) {
        todo!()
    }
}

pub fn start<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>, section: Section) {
    let index = section as usize;
    mcco.borrow_mut().fast_timer.timers[index].start_clock = Instant::now();
}

pub fn stop<T: Float>(mcco: Rc<RefCell<MonteCarlo<T>>>, section: Section) {
    let index = section as usize;
    mcco.borrow_mut().fast_timer.timers[index].end_clock = Instant::now();
    mcco.borrow_mut().fast_timer.timers[index].last_cycle_clock += mcco.borrow().fast_timer.timers
        [index]
        .end_clock
        .duration_since(mcco.borrow().fast_timer.timers[index].start_clock)
        .as_micros();
    mcco.borrow_mut().fast_timer.timers[index].cumulative_clock += mcco.borrow().fast_timer.timers
        [index]
        .end_clock
        .duration_since(mcco.borrow().fast_timer.timers[index].start_clock)
        .as_micros();
    mcco.borrow_mut().fast_timer.timers[index].num_calls += 1;
}

pub fn get_last_cycle<T: Float>(mcco: &MonteCarlo<T>, section: Section) -> f64 {
    let index = section as usize;
    mcco.fast_timer.timers[index].last_cycle_clock as f64 / 1000000.0
}
