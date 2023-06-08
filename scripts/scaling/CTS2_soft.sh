# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Soft scaling study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Problem volume, particles & number of threads all scale together
# Values are taken from Quicksilver's scripts & adapted to our parallel implem.

# target folder
mkdir CTS2_soft_scaling

# 40960 particles -- 16*16*16 mesh -- 1 thread 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
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

# 81920 particles -- 32*16*16 mesh -- 2 threads
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 81920 \
    -X 32 \
    -Y 16 \
    -Z 16 \
    -x 32 \
    -y 16 \
    -z 16 \
    -r 2 \
    -c \

mv tallies_report.csv CTS2_soft_scaling/tallies_r2n81920.csv
mv timers_report.csv CTS2_soft_scaling/timers_r2n81920.csv

# 163840 particles -- 32*32*16 mesh -- 4 threads 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 163840 \
    -X 32 \
    -Y 32 \
    -Z 16 \
    -x 32 \
    -y 32 \
    -z 16 \
    -r 4 \
    -c \

mv tallies_report.csv CTS2_soft_scaling/tallies_r4n163840.csv
mv timers_report.csv CTS2_soft_scaling/timers_r4n163840.csv

# 327680 particles -- 32*32*32 mesh -- 8 threads 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 327680 \
    -X 32 \
    -Y 32 \
    -Z 32 \
    -x 32 \
    -y 32 \
    -z 32 \
    -r 8 \
    -c \

mv tallies_report.csv CTS2_soft_scaling/tallies_r8n327680.csv
mv timers_report.csv CTS2_soft_scaling/timers_r8n327680.csv

# 655360 particles -- 64*32*32 mesh -- 16 threads 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 655360 \
    -X 64 \
    -Y 32 \
    -Z 32 \
    -x 64 \
    -y 32 \
    -z 32 \
    -r 16 \
    -c \

mv tallies_report.csv CTS2_soft_scaling/tallies_r16n655360.csv
mv timers_report.csv CTS2_soft_scaling/timers_r16n655360.csv

# 1310720 particles -- 64*64*32 mesh -- 32 threads
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 1310720 \
    -X 64 \
    -Y 64 \
    -Z 32 \
    -x 64 \
    -y 64 \
    -z 32 \
    -r 32 \
    -c \

mv tallies_report.csv CTS2_soft_scaling/tallies_r32n1310720.csv
mv timers_report.csv CTS2_soft_scaling/timers_r32n1310720.csv

# 1474560 particles -- 48*48*48 mesh -- 36 threads 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 1474560 \
    -X 48 \
    -Y 48 \
    -Z 48 \
    -x 48 \
    -y 48 \
    -z 48 \
    -r 36 \
    -c \

mv tallies_report.csv CTS2_soft_scaling/tallies_r36n1474560.csv
mv timers_report.csv CTS2_soft_scaling/timers_rr36n1474560.csv