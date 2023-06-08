# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Strong scaling study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Only the number of threads scales
# Target number of particles can be specified but a default value is provided

N_PARTICLES=81920

if [ $# -eq 1 ]
  then
    N_PARTICLES=$1
fi

# target folder
TARGET_FOLDER=CTS2_strong_scaling/$N_PARTICLES
mkdir $TARGET_FOLDER

# 81920 particles -- 32*16*16 mesh -- 1 thread 
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n $N_PARTICLES \
    -X 32 \
    -Y 16 \
    -Z 16 \
    -x 32 \
    -y 16 \
    -z 16 \
    -r 1 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r1.csv
mv timers_report.csv $TARGET_FOLDER/timers_r1.csv