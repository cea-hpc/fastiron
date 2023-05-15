# [fastiron]

## Building Fastiron

[fastiron] is a classical Rust program and can be built using `cargo`. Just clone the repository and run `cargo`:

```shell
cargo build --release
```

## Running Fastiron

[fastiron] mimics the original [Quicksilver][quicksilver] executable: they share command line arguments and parameter
files.

```shell
cargo run --bin=fastiron -- -i input_files/QS_originals/AllEscape/allEscape.inp -e energy -S section -n 10000
```

## Outputs

By default, the program will print to the terminal the tallies report and the timers report. Three additional 
options are provided to the user:

- `-c, --csv`: The aforementionned reports will be saved as csv files. The files can directly be used by the 
  stats tool.
- `-e, --energy-spectrum <ENERGY_SPECTRUM>`: Save the endstate energy spectrum in a file named according to
  the specified argument.
- `-S, --cross-sections <CROSS_SECTIONS_OUT`: Save the values of cross-sections of the mesh.

## Features

Fastiron currently implements one feature:

- **single-precision** - When enabled, all computations done during the simulation will be done using the
  single-precision float type `f32`. By default, every computation is done using `f64`.

[fastiron]: https://github.com/cea-hpc/fastiron
[quicksilver]: https://github.com/LLNL/Quicksilver
