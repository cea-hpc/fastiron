#!/usr/local/bin/gnuplot -persist
set terminal pngcairo background rgb 'white' enhanced font "arial,10" fontscale 1.0 size 600, 500
set output 'heatmap_popsync.png'
unset key
unset parametric
set datafile separator comma
set view map scale 1
set style data lines
set xtics border in scale 0,0 mirror norotate  autojustify
set xtics  norangelimit
set xtics   ()
set ytics border in scale 0,0 mirror norotate  autojustify
set ytics  norangelimit
set ytics   ()
set ztics border in scale 0,0 nomirror norotate  autojustify
set cbtics
set size ratio -1
set rtics axis in scale 0,0 nomirror norotate  autojustify
set title "Correlation Matrix"
set xrange [ -0.500000 : 1.50000 ] noreverse nowriteback
set x2range [ * : * ] noreverse writeback
set yrange [ -0.500000 : 1.50000 ] noreverse nowriteback
set y2range [ * : * ] noreverse writeback
set zrange [ -1.0000 : 1.0000 ] noreverse writeback
set cblabel "Correlation"
set cbrange [ -1.00000 : 1.00000 ]
set bmargin 8
set palette defined (-5 0 0 1, 0 1 1 1, 5 1 0 0)
set colorbox horiz user origin graph 0,screen .1 size graph 1,screen .04
NO_ANIMATION = 1

plot 'popsync.dat' matrix rowheaders columnheaders with image, \
     'popsync.dat' matrix rowheaders columnheaders using 1:2:($3 == 0 ? "" : sprintf("%g",$3) ) with labels