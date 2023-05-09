# This is one of two scripts provided to compute some numbers about 
# the behavior of Fastiron

# This script can be used to gather all data used by the analysis code
# to provide insights on the code's behavior. 
# The only additionnal file that is not provided by this script is the 
# previous version's timers report.

PROJECTDIR=$(dirname "$0")/..
DATADIR=$PROJECTDIR/tmp
TARGETDIR=$PROJECTDIR/out
#CARGO=~/.cargo/bin/cargo
STATS=$PROJECTDIR/target/release/fastiron-stats

# Build release
#$CARGO build --release --manifest-path $PROJECTDIR/Cargo.toml
# Create output folder
mkdir -p $TARGETDIR

# Process data gathered by gather_data.sh
$STATS < $PROJECTDIR/profiling/auto_in_stats

# Do the necessary plotting
gnuplot fastiron-stats/heatmap_cycletracking.gnu
gnuplot fastiron-stats/heatmap_popsync.gnu
gnuplot fastiron-stats/linear_scaling.gnu

mv ./*.dat ./out/
mv ./*.png ./out/