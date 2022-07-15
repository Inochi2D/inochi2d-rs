# Inochi2D Rust Bindings

This repository contains the (preliminary) Rust bindings for Inochi2D using the [Inochi2D C SDK](https://github.com/Inochi2D/inochi2d-c).


**NOTE** The bindings pull in a large chunk of the D runtime, including the Garbage Collector.

## Building

To build these bindings, you need [ldc](https://github.com/ldc-developers/ldc), [dub](https://dub.pm/), and [git](https://git-scm.com/) in addition to
the normal rust toolchain.

The [`build.rs`](./build.rs) will attempt to clone [Inochi2D](https://github.com/Inochi2D/inochi2d/) and [Inochi2D-c](https://github.com/Inochi2D/inochi2d-c) into the target directory and build them from source on first build.

You should only need to build the crate as you would normally.

```
$ cargo build
```

## Examples

To build the examples, make sure you have the submodules checked out to ensure the example Inochi2D puppets are where the examples expect them to be.

You can run the main OpenGL example with:
```
$ cargo run --example midori
```
**NOTE:** You need the `libinochi2d-c.so` library in your `LD_LIBRARY_PATH` to run the examples

## License

These bindings are licensed under the [BSD-2-Clause](https://spdx.org/licenses/BSD-2-Clause.html) license, the full text of which can be found in the [LICENSE](./LICENSE) file.
