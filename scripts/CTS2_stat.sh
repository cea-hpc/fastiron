PERF=perf_5.10 # until container is fixed 

$PERF stat -o perf_stat.raw -d fastiron \
    -i ./input_files/profiling/CTS2_sized.inp \
    -N 100 \
    -n 4096000 \
    -r 40 \
    -c \

mv tallies_report.csv CTS2_tallies.csv
mv timers_report.csv CTS2_timers.csv