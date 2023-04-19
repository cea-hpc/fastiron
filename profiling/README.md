# Profiling

---

This folder contains summaries & flamegraphes of the different version of fastiron as they progress. It also 
contains Quicksilver's data for reference. 

## Test problems

The following problems are used in order to compare the versions of the program. Note that these can (and will)
change in the future to fit our needs.

- `homogeneous7`, corresponding to the file `Homogeneous/homogeneousProblem_v7_ts.inp`. Simulation is done two times,
  respectively with `10000`, `100000`
- `CTS2_1`, corresponding to the file `CTS2_Benchmark/CTS2_1.inp`.

## Hardware used for execution

This section contains the hardware specifications of the machine used to obtain this data. 

## Quicksilver modification

In a sequential context, Quicksilver has received a small tweak to guarantee coherence of the results. The random
number generator function used to initialize centers randomly, `drand48`, has been replaced with the program's function,
`rngSample`.