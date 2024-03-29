# This is one of two scripts provided to compute some numbers about 
# the behavior of Fastiron

# This script can be used to process all data gathered by the first script
# The default auto input does not make the comparison study as the "old"
# timers report file is unknown.
# It would be possible to write a script doing this comparison, using a
# single argument for the old timers report.

PROJECTDIR=$(dirname "$0")/../..
DATADIR=$PROJECTDIR/tmp
TARGETDIR=$PROJECTDIR/out
#CARGO=~/.cargo/bin/cargo
STATS=$PROJECTDIR/target/release/fastiron-stats

# Build release
#$CARGO build --release --manifest-path $PROJECTDIR/Cargo.toml
# Create output folder
mkdir -p $TARGETDIR

# Process data gathered by gather_data.sh
$STATS < $(dirname "$0")/auto_in_threads

# Do the necessary plotting
gnuplot $PROJECTDIR/plot/rayon_scaling.gnu

mv $PROJECTDIR/scaling.dat $TARGETDIR/rayon_scaling.dat
mv $PROJECTDIR/scaling_tracking.png $TARGETDIR/threads_scaling_tracking.png

$STATS < $(dirname "$0")/auto_in_stats

# Do the necessary plotting
gnuplot $PROJECTDIR/plot/heatmap_cycletracking.gnu
gnuplot $PROJECTDIR/plot/heatmap_popsync.gnu
gnuplot $PROJECTDIR/plot/linear_scaling.gnu

mv $PROJECTDIR/*.dat $TARGETDIR
mv $PROJECTDIR/*.png $TARGETDIR