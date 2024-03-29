# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# weak scaling study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Problem volume, particles & number of threads all scale together
# Values are taken from Quicksilver's scripts & adapted to our parallel implem.

# target folder
TARGET_FOLDER=CTS2_weak_scaling
mkdir -p $TARGET_FOLDER

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

mv tallies_report.csv $TARGET_FOLDER/tallies_r1.csv
mv timers_report.csv $TARGET_FOLDER/timers_r1.csv

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

mv tallies_report.csv $TARGET_FOLDER/tallies_r2.csv
mv timers_report.csv $TARGET_FOLDER/timers_r2.csv

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

mv tallies_report.csv $TARGET_FOLDER/tallies_r4.csv
mv timers_report.csv $TARGET_FOLDER/timers_r4.csv

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

mv tallies_report.csv $TARGET_FOLDER/tallies_r8.csv
mv timers_report.csv $TARGET_FOLDER/timers_r8.csv

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

mv tallies_report.csv $TARGET_FOLDER/tallies_r16.csv
mv timers_report.csv $TARGET_FOLDER/timers_r16.csv

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

mv tallies_report.csv $TARGET_FOLDER/tallies_r32.csv
mv timers_report.csv $TARGET_FOLDER/timers_r32.csv

# QS original scripts only went up to this for CTS2 benchmark
# 1474560 particles -- 48*48*64 mesh -- 36 threads 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 1474560 \
    -X 48 \
    -Y 48 \
    -Z 64 \
    -x 48 \
    -y 48 \
    -z 64 \
    -r 36 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r36.csv
mv timers_report.csv $TARGET_FOLDER/timers_r36.csv

# 2621440 particles -- 64*64*64 mesh -- 64 threads
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 2621440 \
    -X 64 \
    -Y 64 \
    -Z 64 \
    -x 64 \
    -y 64 \
    -z 64 \
    -r 64 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r64.csv
mv timers_report.csv $TARGET_FOLDER/timers_r64.csv

# 5242880 particles -- 128*64*64 mesh -- 128 threads
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 5242880 \
    -X 128 \
    -Y 64 \
    -Z 64 \
    -x 128 \
    -y 64 \
    -z 64 \
    -r 128 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r128.csv
mv timers_report.csv $TARGET_FOLDER/timers_r128.csv

# 10485760 particles -- 128*128*64 mesh -- 256 threads
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 10485760 \
    -X 128 \
    -Y 128 \
    -Z 64 \
    -x 128 \
    -y 128 \
    -z 64 \
    -r 256 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r256.csv
mv timers_report.csv $TARGET_FOLDER/timers_r256.csv
