# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Strong scaling study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Only the number of threads scales

# target folder
mkdir CTS2_strong_scaling

# 81920 particles -- 32*16*16 mesh -- 2 thread 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 81920 \
    -X 32 \
    -Y 16 \
    -Z 16 \
    -x 32 \
    -y 16 \
    -z 16 \
    -r 1 \
    -c \

mv tallies_report.csv CTS2_strong_scaling/tallies_r1.csv
mv timers_report.csv CTS2_strong_scaling/timers_r1.csv