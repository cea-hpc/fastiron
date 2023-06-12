# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Soft scaling study using Coral2 P2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Problem volume, particles & number of threads all scale together
# Values are taken from Quicksilver's scripts & adapted to our parallel implem.

# target folder
TARGET_FOLDER=Coral2_P2_soft_scaling
mkdir -p $TARGET_FOLDER

# 53240 particles -- 11*11*11 mesh -- 1 thread 
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 53240 \
    -X 1 \
    -Y 1 \
    -Z 1 \
    -x 11 \
    -y 11 \
    -z 11 \
    -r 1 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r1n53240.csv
mv timers_report.csv $TARGET_FOLDER/timers_r1n53240.csv

# 106480 particles -- 22*11*11 mesh -- 2 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 53240 \
    -X 2 \
    -Y 1 \
    -Z 1 \
    -x 22 \
    -y 11 \
    -z 11 \
    -r 2 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r2n106480.csv
mv timers_report.csv $TARGET_FOLDER/timers_r2n106480.csv

# 212960 particles -- 22*22*11 mesh -- 4 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 212960 \
    -X 2 \
    -Y 2 \
    -Z 1 \
    -x 22 \
    -y 22 \
    -z 11 \
    -r 4 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r4n212960.csv
mv timers_report.csv $TARGET_FOLDER/timers_r2n212960.csv

# 425920 particles -- 22*22*22 mesh -- 8 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 425920 \
    -X 2 \
    -Y 2 \
    -Z 2 \
    -x 22 \
    -y 22 \
    -z 22 \
    -r 8 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r8n425920.csv
mv timers_report.csv $TARGET_FOLDER/timers_r8n425920.csv

# 851840 particles -- 44*22*22 mesh -- 16 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 851840  \
    -X 4 \
    -Y 2 \
    -Z 2 \
    -x 44 \
    -y 22 \
    -z 22 \
    -r 16 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r16n851840 .csv
mv timers_report.csv $TARGET_FOLDER/timers_r16n851840 .csv

# 1703680 particles -- 44*44*22 mesh -- 32 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 1703680 \
    -X 4 \
    -Y 4 \
    -Z 2 \
    -x 44 \
    -y 44 \
    -z 22 \
    -r 32 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r32n1703680.csv
mv timers_report.csv $TARGET_FOLDER/timers_r32n1703680.csv

# 3407360 particles -- 44*44*44 mesh -- 64 threads 
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 3407360 \
    -X 4 \
    -Y 4 \
    -Z 4 \
    -x 44 \
    -y 44 \
    -z 44 \
    -r 64 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r64n3407360.csv
mv timers_report.csv $TARGET_FOLDER/timers_r64n3407360.csv

# 6814720 particles -- 88*44*44 mesh -- 128 threads
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 6814720 \
    -X 8 \
    -Y 4 \
    -Z 4 \
    -x 88 \
    -y 44 \
    -z 44 \
    -r 128 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r128n6814720.csv
mv timers_report.csv $TARGET_FOLDER/timers_r128n6814720.csv

# 13629440 particles -- 88*88*44 mesh -- 256 threads
fastiron \
    -i ./input_files/profiling/Coral2_P2.inp \
    -n 13629440 \
    -X 8 \
    -Y 8 \
    -Z 4 \
    -x 88 \
    -y 88 \
    -z 44 \
    -r 256 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_r256n13629440.csv
mv timers_report.csv $TARGET_FOLDER/timers_r256n13629440.csv
