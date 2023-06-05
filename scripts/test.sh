

fastiron -i ./input_files/profiling/short_CTS2.inp -c -n 40960 -r 0 # test the default value of rayon
perf record fastiron -i ./input_files/profiling/one_cycle_CTS2.inp -n 4096000 -r 40