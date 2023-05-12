# Profiling

This folder contains summaries & flamegraphes of the different version of 
fastiron as they progress. It also contains Quicksilver's data for reference. 

Four scripts are provided:

- `bench_cfg.sh`, `reverse_cfg.sh`: Set CPU behavior options to be more consistent 
  when benchmarking on a laptop.
- `gather_data.sh`: Gather profiling data for the current build of [Fastiron][1]. 
- `process_data.sh`: Process the data collected using `gather_data.sh`.

The data gathering / processing done with the script can be tweaked to obtain the desired 
results. Refer to the [Rust Doc][2] or to the `README.md` of `fastiron-stats`. Note that 
**you currently need to build the binaries manually before using the scripts**. This can 
be done easily with cargo: 

```bash
cargo build --release
```

## Sampling policy

All benchmarks have been done using the `bench_cfg.sh` script unless stated otherwise. 
Link-time optimization were not enabled for Fastiron.

Flamegraphes were recorded in the default configuration since the overall distribution of 
time is approximately the same. Additionally, [Quicksilver][3] was recompiled with `-g3` flag, 
and debug information was enabled in the release profile of Fastiron.

## Known issues

- Usage of the data scripts followed by regular usage of the program leads to crashes 
  because of restricted permissions on the created folders.

## Hardware used for execution

Up to the sequential analysis (included), all benchmarking was done on a laptop with the following specs:

| Parts | Model |
|-------|-------|
| CPU   | Intel Core i5-8265U CPU @ 1.60GHz * 8 |
| GPU   | Mesa Intel UHD Graphics 620 (WHL GT2) |
| RAM   | 8GiB SODIMM DDR4 Synchrone 2667 MHz (0,4 ns) |


[1]: https://github.com/cea-hpc/fastiron
[2]: https://cea-hpc.github.io/fastiron/fastiron_stats/
[3]: https://github.com/LLNL/Quicksilver