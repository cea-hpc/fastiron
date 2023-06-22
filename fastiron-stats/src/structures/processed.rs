use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use gnuplot::{
    AutoOption, AxesCommon, DashType, Figure,
    LabelOption::{Rotate, TextOffset},
    PaletteType,
    PlotOption::{Caption, Color, LineStyle},
    Tick,
    TickOption::{Inward, Mirror},
};

use crate::io::command_line::ScalingParams;

use super::raw::{
    correlation, TalliedData, TalliesReport, TimerReport, TimerSV, N_TIMERS, TIMERS_ARR,
};

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
        TIMERS_ARR.iter().for_each(|section| {
            writeln!(
                file,
                "{},{},{},{}",
                *section,
                self.old[*section].total,
                self.new[*section].total,
                self.percents[*section as usize]
            )
            .unwrap();
        });
    }

    pub fn plot(&self) {
        // create figure & adjust characteristics
        let mut fg = Figure::new();
        fg.set_terminal("pngcairo size 800, 800", "comparison.png")
            .set_title("Timers Reports Comparison");
        // prepare data
        let x_coords: [usize; N_TIMERS] = core::array::from_fn(|i| i);
        let x_tics = TIMERS_ARR.map(|section| {
            Tick::Major(
                section as usize,
                AutoOption::Fix(match section {
                    TimerSV::Main => "Main Program",
                    TimerSV::PopulationControl => "Population Control",
                    TimerSV::CycleTracking => "Tracking Phase",
                    TimerSV::CycleTrackingProcess => "Process Phase",
                    TimerSV::CycleTrackingSort => "Sorting Phase",
                    TimerSV::CycleSync => "Sync Phase",
                }),
            )
        });
        let width = 0.25;

        let old_y: [f64; N_TIMERS] = TIMERS_ARR.map(|t| self.old[t].total / 1.0e6);
        let new_y: [f64; N_TIMERS] = TIMERS_ARR.map(|t| self.new[t].total / 1.0e6);
        let new_color = if self.percents[0] > 0.0 {
            Color("#FF0000") // total exec time increase => red
        } else {
            Color("#00BB00") // total exec time decrease => green
        };

        // plot data
        fg.axes2d()
            .set_x_ticks_custom(x_tics, &[Inward(false)], &[Rotate(-45.0)])
            .set_x_label("Section", &[TextOffset(0.0, 1.5)])
            .set_y_grid(true)
            .set_y_label("Total Time Spent in Section (s)", &[])
            .boxes_set_width(
                x_coords.iter().map(|x| *x as f64 - width / 2.0),
                &old_y,
                &[width; N_TIMERS],
                &[Caption("Old times"), Color("#000077")],
            )
            .boxes_set_width(
                x_coords.iter().map(|x| *x as f64 + width / 2.0),
                &new_y,
                &[width; N_TIMERS],
                &[Caption("New times"), new_color],
            );

        fg.show().unwrap();
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
    TalliedData::CycleSync,
    TalliedData::CycleTracking,
    TalliedData::PopulationControl,
    TalliedData::Cycle,
];

pub struct CorrelationResults {
    pub corr_data: [f64; 11 * 4],
}

impl CorrelationResults {
    pub fn save(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("correlation.csv")
            .unwrap();
        writeln!(
            file,
            ",Start,Rr,Split,Absorb,Scatter,Fission,Produce,Collision,Escape,Census,NumSeg"
        )
        .unwrap();
        (0..4).for_each(|idx: usize| {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                match idx {
                    0 => "CycleSync",
                    1 => "CycleTracking",
                    2 => "PopulationControl",
                    3 => "Cycle",
                    _ => unreachable!(),
                },
                self.corr_data[11 * idx],
                self.corr_data[1 + 11 * idx],
                self.corr_data[2 + 11 * idx],
                self.corr_data[3 + 11 * idx],
                self.corr_data[4 + 11 * idx],
                self.corr_data[5 + 11 * idx],
                self.corr_data[6 + 11 * idx],
                self.corr_data[7 + 11 * idx],
                self.corr_data[8 + 11 * idx],
                self.corr_data[9 + 11 * idx],
                self.corr_data[10 + 11 * idx],
            )
            .unwrap();
        });
    }

    pub fn plot(&self) {
        // create figure & adjust characteristics
        let mut fg = Figure::new();
        fg.set_terminal("pngcairo size 1300, 600", "correlation.png")
            .set_title("Event Correlation Matrix");
        // prepare data
        let n_col = CORRELATION_COLS.len();
        let n_row = CORRELATION_ROWS.len();
        let x: [usize; CORRELATION_COLS.len()] = core::array::from_fn(|i| i);
        let y: [usize; CORRELATION_ROWS.len()] = core::array::from_fn(|i| i);
        let x_tics = x.map(|event_idx| {
            Tick::Major(
                event_idx as f64,
                AutoOption::Fix(CORRELATION_COLS[event_idx].to_string()),
            )
        });
        let y_tics = y.map(|event_idx| {
            Tick::Major(
                event_idx as f64,
                AutoOption::Fix(CORRELATION_ROWS[event_idx].to_string()),
            )
        });

        // necessary padding because of the shitty interpreter of .surface() method
        let mut tmp = vec![0.0; n_col + 1];
        (0..n_row).for_each(|row_idx| {
            tmp.push(0.0);
            tmp.extend_from_slice(&self.corr_data[row_idx * n_col..(row_idx + 1) * n_col]);
        });

        fg.axes3d()
            .set_aspect_ratio(AutoOption::Fix(0.35))
            .set_x_ticks_custom(
                x_tics,
                &[Inward(false), Mirror(false)],
                &[Rotate(-45.0), TextOffset(5.0, -1.0)],
            )
            .set_y_ticks_custom(
                y_tics,
                &[Inward(false), Mirror(false)],
                &[Rotate(45.0), TextOffset(0.0, 2.0)],
            )
            .set_cb_grid(true)
            .set_x_grid(true)
            .set_y_grid(true)
            .set_grid_options(true, &[LineStyle(DashType::Solid)])
            .set_palette(PaletteType::Custom(vec![
                (-5.0, 0.0, 0.0, 1.0),
                (0.0, 1.0, 1.0, 1.0),
                (5.0, 1.0, 0.0, 0.0),
            ]))
            .set_grid_options(true, &[LineStyle(DashType::Solid)])
            .surface(&tmp, n_row + 1, n_col + 1, None, &[Caption("%f.2")])
            .set_view_map();

        fg.show().unwrap();
    }
}

impl From<TalliesReport> for CorrelationResults {
    fn from(report: TalliesReport) -> Self {
        // compute correlations
        let table: [[f64; 11]; 4] = CORRELATION_ROWS.map(|tallied_data| {
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

//~~~~~~~~~~~~~~
// Scaling data
//~~~~~~~~~~~~~~

pub enum ScalingType {
    Weak,
    Strong(usize),
}

pub struct ScalingResults {
    pub n_threads: Vec<usize>,
    pub total_exec_times: Vec<f64>,
    pub population_control_avgs: Vec<f64>,
    pub tracking_avgs: Vec<f64>,
    pub tracking_process_avgs: Vec<f64>,
    pub tracking_sort_avgs: Vec<f64>,
    pub sync_avgs: Vec<f64>,
    pub scaling_type: ScalingType,
}

impl ScalingResults {
    pub fn save_tracking(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(match self.scaling_type {
                ScalingType::Weak => "weak_scaling_tracking.csv",
                ScalingType::Strong(_) => "strong_scaling_tracking.csv",
            })
            .unwrap();
        writeln!(
            file,
            "n_threads,TrackingAvg,TrackingAvgIdeal,TrackingProcessAvg,TrackingSortAvg"
        )
        .unwrap();
        let avg_ref = self.tracking_avgs[0];
        let n_elem = self.n_threads.len();
        assert_eq!(self.tracking_avgs.len(), n_elem);
        assert_eq!(self.tracking_process_avgs.len(), n_elem);
        assert_eq!(self.tracking_sort_avgs.len(), n_elem);
        for idx in 0..n_elem {
            let ideal = match self.scaling_type {
                ScalingType::Weak => avg_ref,
                ScalingType::Strong(factor) => avg_ref / (factor.pow(idx as u32) as f64),
            };
            writeln!(
                file,
                "{},{},{},{},{}",
                self.n_threads[idx],
                self.tracking_avgs[idx],
                ideal,
                self.tracking_process_avgs[idx],
                self.tracking_sort_avgs[idx]
            )
            .unwrap();
        }
    }

    pub fn save_others(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(match self.scaling_type {
                ScalingType::Weak => "weak_scaling_others.csv",
                ScalingType::Strong(_) => "strong_scaling_others.csv",
            })
            .unwrap();
        writeln!(file, "n_threads,TotalExecTime,PopulationControlAvg,SyncAvg").unwrap();
        let n_elem = self.n_threads.len();
        assert_eq!(self.total_exec_times.len(), n_elem);
        assert_eq!(self.population_control_avgs.len(), n_elem);
        assert_eq!(self.sync_avgs.len(), n_elem);
        for idx in 0..n_elem {
            writeln!(
                file,
                "{},{},{},{}",
                self.n_threads[idx],
                self.total_exec_times[idx],
                self.population_control_avgs[idx],
                self.sync_avgs[idx]
            )
            .unwrap();
        }
    }

    pub fn plot_tracking(&self) {
        todo!()
    }

    pub fn plot_others(&self) {
        todo!()
    }
}

impl From<(&str, &ScalingParams, ScalingType)> for ScalingResults {
    fn from((root_path, params, scaling_type): (&str, &ScalingParams, ScalingType)) -> Self {
        // fetch data from files
        let n_threads: Vec<usize> = (0..params.t_iter.unwrap())
            .map(|idx| params.t_init.unwrap() * params.t_factor.unwrap().pow(idx as u32))
            .collect();
        let reports: Vec<TimerReport> = n_threads
            .iter()
            .map(|n_thread| {
                let filename = format!("{}{}.csv", root_path, n_thread);
                TimerReport::from(File::open(filename).unwrap())
            })
            .collect();

        // use data to init structure
        let mut total_exec_times: Vec<f64> = Vec::with_capacity(n_threads.len());
        let mut population_control_avgs: Vec<f64> = Vec::with_capacity(n_threads.len());
        let mut tracking_avgs: Vec<f64> = Vec::with_capacity(n_threads.len());
        let mut tracking_process_avgs: Vec<f64> = Vec::with_capacity(n_threads.len());
        let mut tracking_sort_avgs: Vec<f64> = Vec::with_capacity(n_threads.len());
        let mut sync_avgs: Vec<f64> = Vec::with_capacity(n_threads.len());

        reports.iter().for_each(|report| {
            total_exec_times.push(report[TimerSV::Main].mean);
            population_control_avgs.push(report[TimerSV::PopulationControl].mean);
            tracking_avgs.push(report[TimerSV::CycleTracking].mean);
            tracking_process_avgs.push(report[TimerSV::CycleTrackingProcess].mean);
            tracking_sort_avgs.push(report[TimerSV::CycleTrackingSort].mean);
            sync_avgs.push(report[TimerSV::CycleSync].mean);
        });

        Self {
            n_threads,
            total_exec_times,
            population_control_avgs,
            tracking_avgs,
            tracking_process_avgs,
            tracking_sort_avgs,
            sync_avgs,
            scaling_type,
        }
    }
}
