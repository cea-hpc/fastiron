# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Chunk size effects study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Setup is as follows:


# 10485760 particles -- 64*64*64 mesh -- 64 threads 
# This is basically the same density as the soft scaling case, adapted for 40 threads usage.

# Chunk size ranges from 64 up to (n_particles/n_threads).log2().ceil(),
# basically the lowest power of 2 so that the number of chunks is equal or inferior to the number
# threads. In our case, this is 2^18=262144 since n_particles/n_threads=163840.
# We run a simulation for every power of 2 in the range as well as the exact chunk size yielding
# one chunk per thread for n_particles (this corresponds to 163840 here).

# target folder
TARGET_FOLDER=Coral2_chunk_size/staticP1
mkdir -p $TARGET_FOLDER

# 64-particles-chunks
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
    -C 64 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C64.csv
mv timers_report.csv $TARGET_FOLDER/timers_C64.csv

# 128-particles-chunks
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
    -C 128 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C128.csv
mv timers_report.csv $TARGET_FOLDER/timers_C128.csv

# 256-particles-chunks
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
    -C 256 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C256.csv
mv timers_report.csv $TARGET_FOLDER/timers_C256.csv

# 512-particles-chunks
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
    -C 512 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C512.csv
mv timers_report.csv $TARGET_FOLDER/timers_C512.csv

# 1024-particles-chunks
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
    -C 1024 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C1024.csv
mv timers_report.csv $TARGET_FOLDER/timers_C1024.csv

# 2048-particles-chunks
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
    -C 2048 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C2048.csv
mv timers_report.csv $TARGET_FOLDER/timers_C2048.csv

# 4096-particles-chunks
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
    -C 4096 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C4096.csv
mv timers_report.csv $TARGET_FOLDER/timers_C4096.csv

# 8192-particles-chunks
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
    -C 8192 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C8192.csv
mv timers_report.csv $TARGET_FOLDER/timers_C8192.csv

# 16384-particles-chunks
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
    -C 16384 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C16384.csv
mv timers_report.csv $TARGET_FOLDER/timers_C16384.csv

# 32768-particles-chunks
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
    -C 32768 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C32768.csv
mv timers_report.csv $TARGET_FOLDER/timers_C32768.csv

# 65536-particles-chunks
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
    -C 65536 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C65536.csv
mv timers_report.csv $TARGET_FOLDER/timers_C65536.csv

# 131072-particles-chunks
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
    -C 131072 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C131072.csv
mv timers_report.csv $TARGET_FOLDER/timers_C131072.csv

# 163840-particles-chunks
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
    -C 163840 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C163840.csv
mv timers_report.csv $TARGET_FOLDER/timers_C163840.csv


# 262144-particles-chunks
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
    -C 262144 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C262144.csv
mv timers_report.csv $TARGET_FOLDER/timers_C262144.csv
