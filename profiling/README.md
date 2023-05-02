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

Up to the sequential analysis (included), all benchmarking was done on a laptop with the following specs:

| Parts | Model |
|-------|-------|
| CPU   | Intel Core i5-8265U CPU @ 1.60GHz * 8 |
| GPU   | Mesa Intel UHD Graphics 620 (WHL GT2) |
| RAM   | 8GiB SODIMM DDR4 Synchrone 2667 MHz (0,4 ns) |

## Quicksilver modification

In a sequential context, Quicksilver has received a small tweak to guarantee coherence of the results. The random
number generator function used to initialize centers randomly, `drand48`, has been replaced with the program's function,
`rngSample`.