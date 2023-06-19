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
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/current_timers.csv
mv $PROJECTDIR/tallies_report.csv $TARGETDIR/CTS2_tallies.csv

# Run scaling simulations
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 10000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers10000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 20000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers20000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 30000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers30000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 40000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers40000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 50000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers50000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 60000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers60000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 70000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers70000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 80000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers80000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 90000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers90000.csv
$FASTIRON -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers100000.csv
