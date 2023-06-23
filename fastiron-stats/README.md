# Statistical Analysis of [Fastiron][1]

This repository contains code used for statistical analysis of the behavior
of [Fastiron][1], a Monte-Carlo particle transport code written in Rust.

## Usage

The program can be run like any other cargo projects: 

```
cargo run --release
```

The executable uses command line arguments for both data input and control flow. There 
are currently three supported computations:

- **Version comparison**: Simple relative differences computation between two timer
  reports. This is useful to have numbers quickly and easily. The presented 
  results are percentages and **positiveness / negativeness have meaning**. 
- **Correlation study**: Computes correlation coefficients between tallied events 
  and section lengths. The results are formatted in a `.csv` file and can be plotted 
  using the `-p` argument.
- **Scaling study**: Compiles data from a collection of timer to a `.csv` file. The 
  compiled data can be plotted using the `-p` argument.

Refer to the Rust doc pages for more information about the command line usage.

### Comparison Study

The user will be asked to provide two timers `.csv` files, referred to as the 
_old_ one and the _new_ one (read _previous_ and _current_ in the context of 
versions). Percents will be computed using [this][3] definition and saved in 
a `.csv` file.

### Benchmark Statistics

The user will be asked to provide a single tallies report file. From the values
of this file will be built random variables, each taking a value according to
the cycle index. Specific correlation coefficients are then computed to evaluate 
the influence of tallied event on the execution time of each main sections: 
`PopulationControl`, `CycleTracking` and `CycleSync`.

### Scaling Graph

The user will be asked to provide four parameters for the program to run correctly:

- The common root name of the multiple timers files
- The number of threads used for the first simulation
- The number of iteration, i.e. the number of samples
- The multiplying factor defining the progression

Here is an example:

```
+>workspace
|
+--+>some_folder_with_data/
|  |
|  +--+timers_r1.csv
|     +timers_r2.csv
|     +timers_r8.csv
|     +timers_r4.csv
In this case:
 - the root is "some_folder_with_data/timers_r".
 - the starting number of threads is 1
 - the number of iteration is 4
 - the multiplying factor is 2
```

## References

- Fastiron [repository][1]
- `gnuplot` heatmap [examples][2]
- Relative difference [definition][3]
- `gnuplot` Rust [bindings][4]

[1]: https://github.com/cea-hpc/fastiron
[2]: https://gnuplot.sourceforge.net/demo/heatmaps.html
[3]: https://en.wikipedia.org/wiki/Relative_change_and_difference#Definition
[4]: https://docs.rs/gnuplot/latest/gnuplot/