# script used to gather scaling data using 
# a the homogeneous7 example

# should be called from root folder of the project

FI=./target/release/fastiron

cargo build --release
mkdir -p ./profiling/SequentialAnalysis/scaling_data/
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 10000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers10000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies10000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 20000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers20000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies20000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 30000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers30000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies30000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 40000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers40000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies40000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 50000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers50000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies50000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 60000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers60000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies60000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 70000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers70000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies70000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 80000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers80000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies80000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 90000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers90000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies90000.csv
$FI -i input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv ./timers_report.csv ./profiling/SequentialAnalysis/scaling_data/timers100000.csv
mv ./tallies_report.csv ./profiling/SequentialAnalysis/scaling_data/tallies100000.csv

