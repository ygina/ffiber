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

