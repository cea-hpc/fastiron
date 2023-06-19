# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Chunk size effects study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Setup is as follows:

# 2621440 particles -- 64*64*64 mesh -- 64 threads
# This is basically the same density as the soft scaling case, adapted for 64 threads usage.

# Chunk size ranges from 64 up to (n_particles/n_threads).log2().ceil(),
# basically the lowest power of 2 so that the number of chunks is equal or inferior to the number
# threads. In our case, this is 2^16=65536 since n_particles/n_threads=40960.
# We run a simulation for every power of 2 in the range as well as the exact chunk size yielding
# one chunk per thread for n_particles (this corresponds to 40960 here).

echo "~~~~~~~~~~~NON FUNCTIONNAL~~~~~~~~~~~"

# target folder
TARGET_FOLDER=CTS2_chunk_size/static
mkdir -p $TARGET_FOLDER

# 64-particles-chunks
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
    -C 64 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C64.csv
mv timers_report.csv $TARGET_FOLDER/timers_C64.csv

# 128-particles-chunks
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
    -C 128 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C128.csv
mv timers_report.csv $TARGET_FOLDER/timers_C128.csv

# 256-particles-chunks
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
    -C 256 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C256.csv
mv timers_report.csv $TARGET_FOLDER/timers_C256.csv

# 512-particles-chunks
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
    -C 512 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C512.csv
mv timers_report.csv $TARGET_FOLDER/timers_C512.csv

# 1024-particles-chunks
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
    -C 1024 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C1024.csv
mv timers_report.csv $TARGET_FOLDER/timers_C1024.csv

# 2048-particles-chunks
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
    -C 2048 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C2048.csv
mv timers_report.csv $TARGET_FOLDER/timers_C2048.csv

# 4096-particles-chunks
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
    -C 4096 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C4096.csv
mv timers_report.csv $TARGET_FOLDER/timers_C4096.csv

# 8192-particles-chunks
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
    -C 8192 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C8192.csv
mv timers_report.csv $TARGET_FOLDER/timers_C8192.csv

# 16384-particles-chunks
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
    -C 16384 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C16384.csv
mv timers_report.csv $TARGET_FOLDER/timers_C16384.csv

# 32768-particles-chunks
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
    -C 32768 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C32768.csv
mv timers_report.csv $TARGET_FOLDER/timers_C32768.csv

# 40960-particles-chunks
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
    -C 40960 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C40960.csv
mv timers_report.csv $TARGET_FOLDER/timers_C40960.csv

# 65536-particles-chunks
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
    -C 65536 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C65536.csv
mv timers_report.csv $TARGET_FOLDER/timers_C65536.csv
