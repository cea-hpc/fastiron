# This is one of two scripts provided to compute some numbers about 
# the behavior of Fastiron

# This script can be used to gather all data used by the analysis code
# to provide insights on the code's behavior. 
# The only additionnal file that is not provided by this script is the 
# previous version's timers report.

PROJECTDIR=$(dirname "$0")/../..
TARGETDIR=$PROJECTDIR/tmp
#CARGO=~/.cargo/bin/cargo
FASTIRON=$PROJECTDIR/target/release/fastiron

# Build release
#$CARGO build --release --manifest-path $PROJECTDIR/Cargo.toml
# Create output folder
mkdir -p $TARGETDIR

# Run CTS2 benchmark & save results
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/current_timers.csv
mv $PROJECTDIR/tallies_report.csv $TARGETDIR/CTS2_tallies.csv

# Run scaling simulations
$FASTIRON -r 1 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/threads1.csv
$FASTIRON -r 2 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/threads2.csv
$FASTIRON -r 4 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/threads4.csv
$FASTIRON -r 8 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/threads8.csv
$FASTIRON -r 16 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/threads16.csv

# Run scaling simulations
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 10000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles10000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 20000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles20000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 30000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles30000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 40000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles40000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 50000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles50000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 60000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles60000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 70000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles70000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 80000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles80000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 90000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles90000.csv
$FASTIRON -r 0 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/particles100000.csv
