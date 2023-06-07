
mkdir CTS2_soft_scaling

fastiron \
    -i ./input_files/profiling/CTS2_sized.inp \
    -n 40960 \
    -X 16 \
    -Y 16 \
    -Z 16 \
    -x 16 \
    -y 16 \
    -z 16 \
    -r 1 \
    -c \

mv tallies_report.csv CTS2_soft_scaling/tallies_r1n40960.csv
mv timers_report.csv CTS2_soft_scaling/timers_r1n40960.csv