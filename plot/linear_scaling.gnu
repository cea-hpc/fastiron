# global config
set terminal pngcairo background rgb 'white' enhanced font "arial,10" fontscale 1.0
set datafile separator comma
set grid
set xlabel "Target number of particles"
set xrange [ 0 : 110000 ] noreverse nowriteback
set ylabel "Time spent in section (µs)"

# PopulationControl
set title "PopulationControl Average Time = f(n_{particles})"
set output 'scaling_ppcontrol.png'
plot 'scaling.dat' using 1:2 notitle with linespoints

# CycleTracking
set yrange [ 0 : 1150000 ] noreverse nowriteback
set title "CycleTracking Average Time = f(n_{particles})"
set output 'scaling_tracking.png'
plot 'scaling.dat' using 1:3 notitle with linespoints

# CycleSync
set yrange [ 600 : 1800 ] noreverse nowriteback
set title "CycleSync Average Time = f(n_{particles})"
set output 'scaling_sync.png'
plot 'scaling.dat' using 1:4 notitle with linespoints