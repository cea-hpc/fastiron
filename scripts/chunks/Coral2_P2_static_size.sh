# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Chunk size effects study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Setup is as follows:

# 1105920 particles -- 55*55*55 mesh -- 40 threads
# This is basically the same density as the soft scaling case, adapted for 40 threads usage.

# Chunk size ranges from 64 up to (n_particles/n_threads).log2().ceil(),
# basically the lowest power of 2 so that the number of chunks is equal or inferior to the number
# threads. In our case, this is 2^15=32768 since n_particles/n_threads=27648.
# We run a simulation for every power of 2 in the range as well as the exact chunk size yielding
# one chunk per thread for n_particles (this corresponds to 27649 here).

# target folder
TARGET_FOLDER=CTS2_chunk_size/static
mkdir -p $TARGET_FOLDER

# 64-particles-chunks
fastiron \
    -i ./input_files/profiling/CTS2.inp \
    -n 1105920 \
    -X 55 \
    -Y 55 \
    -Z 55 \
    -x 55 \
    -y 55 \
    -z 55 \
    -r 40 \
    -C 64 \
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C64.csv
mv timers_report.csv $TARGET_FOLDER/timers_C64.csv
