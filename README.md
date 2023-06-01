# [fastiron]

## Introduction

[fastiron] is a simplified Monte-Carlo particle transport simulation code used 
to study their behavior on various hardware architectures. It started as a port 
of the [Quicksilver][quicksilver] mini app in Rust.

Currently, [fastiron] aims at studying shared-memory parallelism and does not 
implement any distributed parallelism. The main focus of this project is to 
evaluate the capabilities of the Rust programming language

The main program produces outputs that can be analyzed using the `fastiron-stats`
binary. Detailed explanation on its usage can be found in its folder.

A few scripts are provided for gathering data and processing it using the
custom tool. You can refer to the Rust Doc or to the `README.md` files in each
sub-folder.

## Trying with a container

Building the container:

```shell
podman build . -t fastiron
```

Running the fastiron:
```shell
podman run -v $PWD:/fastiron fastiron fastiron -i input_files/QS_originals/CTS2_1.inp
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

The [SPDX](https://spdx.dev) license identifier for this project is `MIT OR Apache-2.0`.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


[fastiron]: https://github.com/cea-hpc/fastiron

[quicksilver]: https://github.com/LLNL/Quicksilver

