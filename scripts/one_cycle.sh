PERF=perf_5.10 # until container is fixed 

$PERF record fastiron \
    -i ./input_files/profiling/OCCTS2.inp \
    -N 1 \
    -n 4096000 \
    -r 40 \
    -c
$PERF stat -d fastiron \
    -i ./input_files/profiling/OCCTS2.inp \
    -N 1 \
    -n 4096000 \
    -r 40 \
    -c