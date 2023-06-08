# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Chunk size effects study using CTS2 bench specs
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Setup is as follows:
# 1105920 particles -- 55*55*55 mesh -- 40 threads
# This is basically the same density as the soft scaling case, adapted for 40 threads usage.

TARGET_FOLDER=CTS2_chunk_size/static
mkdir -p $TARGET_FOLDER

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
    -C 64
    -c \

mv tallies_report.csv $TARGET_FOLDER/tallies_C64.csv
mv timers_report.csv $TARGET_FOLDER/timers_C64.csv