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
