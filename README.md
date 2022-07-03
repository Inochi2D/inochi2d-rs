# Inochi2D Rust Bindings

This repository contains the (preliminary) Rust bindings for Inochi2D using the [Inochi2D C SDK](https://github.com/Inochi2D/inochi2d-c).


**NOTE** The bindings pull in a large chunk of the D runtime, including the Garbage Collector.

## Building

These bindings currently assume the build `libinochi2d-c.so` is located in the root of the repository.

Then just run `cargo build`

## License

These bindings are licensed under the [BSD-2-Clause](https://spdx.org/licenses/BSD-2-Clause.html) license, the full text of which can be found in the [LICENSE](./LICENSE) file.
