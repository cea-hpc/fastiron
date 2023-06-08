# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Soft scaling study using Coral2 P1 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Only the number of threads scales
# Target number of particles can be specified but a default value is provided.
# The default value correspond to the fixed size of the problem

N_PARTICLES=425920

if [ $# -eq 1 ]
  then
    N_PARTICLES=$1
fi

# target folder
TARGET_FOLDER=Coral2_P2_strong_scaling/$N_PARTICLES
mkdir -p $TARGET_FOLDER

# 22*22*22 mesh -- 1 thread 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 1 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r1.csv
mv timers_report.csv $TARGET_FOLDER/timers_r1.csv

# 22*22*22 mesh -- 2 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 2 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r2.csv
mv timers_report.csv $TARGET_FOLDER/timers_r2.csv

# 22*22*22 mesh -- 4 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 4 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r4.csv
mv timers_report.csv $TARGET_FOLDER/timers_r4.csv

# 22*22*22 mesh -- 8 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 8 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r8.csv
mv timers_report.csv $TARGET_FOLDER/timers_r8.csv

# 22*22*22 mesh -- 16 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 16 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r16.csv
mv timers_report.csv $TARGET_FOLDER/timers_r16.csv

# 22*22*22 mesh -- 32 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 32 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r32.csv
mv timers_report.csv $TARGET_FOLDER/timers_r32.csv

# 22*22*22 mesh -- 64 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 64 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r64.csv
mv timers_report.csv $TARGET_FOLDER/timers_r64.csv

# 22*22*22 mesh -- 128 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2_sized.inp \
    -n $N_PARTICLES \
    -r 128 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r128.csv
mv timers_report.csv $TARGET_FOLDER/timers_r128.csv
