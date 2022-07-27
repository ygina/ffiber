pub mod types;

use color_eyre::eyre::Result;
use types::{ArgType, SelfArgType, DerivedTrait};

pub struct CDylibCompiler {
}

impl CDylibCompiler {
    /// Initialize a compiler for generating a cdylib crate at the given output
    /// folder. The compiler will also generate a build.rs file that uses
    /// cbindgen to output a header file.
    pub fn new(
        _package_name: &str,
        _output_folder: &str,
    ) -> Self {
        unimplemented!()
    }

    /// Add a crate with a specific version (or path to the crate) to the
    /// Cargo.toml.
    pub fn add_crate(
        &mut self,
        _crate_name: &str,
        _version: &str,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Add a dependency to the generated .rs file e.g., `use <dependency>`.
    pub fn add_dependency(&mut self, _dependency: &str) -> Result<()> {
        unimplemented!()
    }

    /// Adds an extern C function wrapper around a method on a struct.
    ///
    /// Params:
    /// - `extern_name`: The extern function name, <extern_name>_<struct_name>
    ///    by default.
    /// - `struct_name`: The struct name on which the function is defined.
    /// - `func_call`: The name of the function call on the struct.
    /// - `self_ty`: If the function call has a self argument, whether it is
    ///    mutable and/or a reference.
    /// - `raw_args`: The names and types of the function call arguments.
    /// - `raw_ret`: The function call return value type, if there is one.
    /// - `use_error_code`: Whether the function call returns a Result.
    ///
    /// TODO: Parse these options directly from the function specification.
    pub fn add_extern_c_function(
        &mut self,
        _extern_name: Option<&str>,
        _struct_name: &str,
        _func_call: &str,
        _self_ty: Option<SelfArgType>,
        _raw_args: Vec<(&str, ArgType)>,
        _raw_ret: Option<ArgType>,
        _use_error_code: bool,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Adds functions for a C void pointer representing a Rust struct.
    ///
    /// Params:
    /// - struct_name: The name of the Rust struct.
    ///
    /// TODO: Generate getters and setters for public fields.
    /// TODO: Generate derived trait functions.
    /// TODO: Parse these options directly from the struct definition.
    pub fn add_opaque_struct(
        &mut self,
        _struct_name: &str,
        _fields: Vec<(&str, ArgType)>,
        _traits: Vec<DerivedTrait>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Writes the cdylib crate to the output folder and runs rustfmt.
    pub fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}
