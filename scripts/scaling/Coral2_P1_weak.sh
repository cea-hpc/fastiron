# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# weak scaling study using Coral2 P1 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Problem volume, particles & number of threads all scale together
# Values are taken from Quicksilver's scripts & adapted to our parallel implem.

# target folder
TARGET_FOLDER=Coral2_P1_weak_scaling
mkdir -p $TARGET_FOLDER

# 163840 particles -- 16*16*16 mesh -- 1 thread 
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 163840 \
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

# 327680 particles -- 32*16*16 mesh -- 2 threads
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 327680 \
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

# 655360 particles -- 32*32*16 mesh -- 4 threads
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 655360 \
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

# 1310720 particles -- 32*32*32 mesh -- 8 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 1310720 \
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

# 2621440 particles -- 64*32*32 mesh -- 16 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 2621440 \
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

# 5242880 particles -- 64*64*32 mesh -- 32 threads
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 5242880 \
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

# 10485760 particles -- 64*64*64 mesh -- 64 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 10485760 \
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

# 20971520 particles -- 128*64*64 mesh -- 128 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 20971520 \
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

# 41943040 particles -- 128*128*64 mesh -- 256 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P1.inp \
    -n 41943040 \
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
