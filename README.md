# ffiber

_ffiber_ generates C bindings from Rust libraries with Rust-y function
interfaces.

When calling C from Rust, the more common direction, it is recommended to write
[a safe wrapping module](https://anssi-fr.github.io/rust-guide/07_ffi.html#FFI-SAFEWRAPPING)
around low-level C bindings to ensure memory safety and security invariants at
the Rust level. When calling Rust from C, Rust library developers should first
write their libraries with a safe, Rust-y interface. They can then use _ffiber_
to programmatically generate low-level extern C functions, and a tool like
[cbindgen](https://github.com/eqrion/cbindgen) to generate the C header file.

## How to Use

ffiber is currently only available as a library (such as in your build.rs).

1. Initialize a `CDylibCompiler`.
2. Use `add_dependency()` to add crate dependencies, including the library you
are trying to bind.
3. Use `import()` to import the necessary crates, structs, traits, functions
etc. to the generated src/lib.rs.
4. Finally, use `add_extern_c_function()` to generate wrappers around library
functions based on their specifications.

If necessary, you can also use the inner `SerializationCompiler` to manually
generate code. This may be helpful, for example, to create intermediate
wrapper functions for unimplemented features such as tuple arguments.

Executing this code will generate a crate at your specified path. This crate
has `cbindgen` as a build dependency by default. Build this crate to generate a
C header file at its root.

## Example

See `examples/`.

```
cargo r --release cornflakes
cd mlx5-datapath-c
cargo b  # mlx5_datapath.h
```

## Roadmap

Several features are [planned](https://github.com/ygina/ffiber/issues). Comment
on an issue if you'd like to help.
