mod codegen;
pub mod compiler;
pub mod types;

use std::{fs, path::{Path, PathBuf}};
use color_eyre::eyre::{Result, ErrReport};
use types::{Type, SelfType, DerivedTrait};

pub struct CDylibCompiler {
    pub inner: compiler::SerializationCompiler,
    pub package_name: String,
    pub package_name_c: String,
    pub package_folder: PathBuf,
    crates: Vec<String>,
}

impl CDylibCompiler {
    /// Initialize a compiler for generating a cdylib crate at the given output
    /// folder. The compiler will also generate a build.rs file that uses
    /// cbindgen to output a header file.
    pub fn new(
        package_name: &str,
        output_folder: &str,
    ) -> Self {
        let package_name = str::replace(package_name, "-", "_");
        let package_name_c =
            format!("{}-c", str::replace(&package_name, "_", "-"));
        CDylibCompiler {
            inner: compiler::SerializationCompiler::new(),
            package_folder:
                Path::new(output_folder).join(&package_name_c).to_path_buf(),
            package_name,
            package_name_c,
            crates: vec![],
        }
    }

    /// Add a specific version of a crate to the Cargo.toml.
    pub fn add_dependency_version(
        &mut self,
        crate_name: &str,
        version: &str,
    ) {
        self.crates.push(format!("{} = \"{}\"", crate_name, version));
    }

    /// Add a crate to the Cargo.toml.
    pub fn add_dependency(
        &mut self,
        crate_name: &str,
        kv: Vec<(&str, &str)>,
    ) -> Result<()> {
        if !kv.is_empty() {
            self.crates.push(format!(
                "{} = {{ {} }}",
                crate_name,
                kv.into_iter().map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<_>>().join(","),
            ));
            Ok(())
        } else {
            Err(ErrReport::msg("Cargo.toml crate requires at least one key."))
        }
    }

    /// Add a dependency to the generated .rs file e.g., `use <dependency>`.
    pub fn import(&mut self, dependency: &str) -> Result<()> {
        self.inner.add_dependency(dependency)?;
        Ok(())
    }

    /// Adds an extern C function wrapper around a method on a struct.
    /// The default name is <struct_name>_<func_call>, where the name of the
    /// struct omits the parameterized types.
    ///
    /// Params:
    /// - `struct_ty`: The struct on which the function is defined.
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
        struct_ty: Type,
        self_ty: SelfType,
        func_call: &str,
        raw_args: Vec<(&str, Type)>,
        raw_ret: Option<Type>,
        use_error_code: bool,
    ) -> Result<()> {
        assert!(struct_ty.is_struct());
        let struct_name = match struct_ty {
            Type::Struct { ref name, .. } => name,
            _ => unreachable!(),
        };
        codegen::add_extern_c_function(
            &mut self.inner,
            &format!("{}_{}", struct_name, func_call),
            Some((struct_ty, self_ty)),
            func_call,
            raw_args,
            raw_ret,
            use_error_code,
        )?;
        Ok(())
    }

    /// Like `add_extern_c_function` except overrides the extern function name.
    pub fn add_extern_c_function_with_name(
        &mut self,
        extern_name: &str,
        struct_ty: Type,
        self_ty: SelfType,
        func_call: &str,
        raw_args: Vec<(&str, Type)>,
        raw_ret: Option<Type>,
        use_error_code: bool,
    ) -> Result<()> {
        assert!(struct_ty.is_struct());
        codegen::add_extern_c_function(
            &mut self.inner,
            extern_name,
            Some((struct_ty, self_ty)),
            func_call,
            raw_args,
            raw_ret,
            use_error_code,
        )?;
        Ok(())
    }

    /// Like `add_extern_c_function` except it is a wrapper around a standalone
    /// function rather than a function defined on a struct.
    pub fn add_extern_c_function_standalone(
        &mut self,
        extern_name: &str,
        func_call: &str,
        raw_args: Vec<(&str, Type)>,
        raw_ret: Option<Type>,
        use_error_code: bool,
    ) -> Result<()> {
        codegen::add_extern_c_function(
            &mut self.inner,
            extern_name,
            None,
            func_call,
            raw_args,
            raw_ret,
            use_error_code,
        )?;
        Ok(())
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
        _fields: Vec<(&str, Type)>,
        _traits: Vec<DerivedTrait>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Writes the cdylib crate to the output folder and runs rustfmt.
    ///
    /// package-name-c/
    ///     src/
    ///         lib.rs
    ///     build.rs
    ///     Cargo.toml
    pub fn flush(&mut self) -> Result<()> {
        let src_folder = self.package_folder.join("src");
        fs::create_dir_all(&src_folder)?;

        codegen::gen_build_rs(&self.package_name, &self.package_folder)?;
        codegen::gen_cargo_toml(&self.package_name, &self.package_folder,
            &self.crates)?;

        let lib_file = src_folder.join("lib.rs");
        self.inner.flush(&lib_file)?;
        compiler::run_rustfmt(&lib_file)?;
        Ok(())
    }
}
