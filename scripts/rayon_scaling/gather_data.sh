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

# Run scaling simulations
$FASTIRON -r 1 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers1.csv
$FASTIRON -r 2 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers2.csv
$FASTIRON -r 4 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers4.csv
$FASTIRON -r 8 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers8.csv
$FASTIRON -r 16 -i $PROJECTDIR/input_files/QS_originals/Homogeneous/homogeneousProblem_v7_ts.inp -n 100000 -c
mv $PROJECTDIR/timers_report.csv $TARGETDIR/timers16.csv
