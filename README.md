# [fastiron]

## Introduction

[fastiron] is a port of the [Quicksilver][quicksilver] mini app in Rust. It mimics Monte-Carlo particle transport
simulation codes to study their behavior on various hardware architectures.

Currently, [fastiron] aims at studying shared-memory parallelism and does not implement any distributed parallelism.

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

## Contributing

Contributions are welcome and accepted as pull requests on [GitHub][fastiron].

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


[fastiron]: https://github.com/cea-hpc/fastiron

[quicksilver]: https://github.com/LLNL/Quicksilver

