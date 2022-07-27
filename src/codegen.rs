use super::compiler::{Context, FunctionContext, SerializationCompiler};
use color_eyre::eyre::Result;
use std::{str, path::Path};

pub fn gen_build_rs(package_name: &str, package_folder: &Path) -> Result<()> {
    let mut compiler = SerializationCompiler::new();
    compiler.add_extern_crate("cbindgen")?;
    compiler.add_newline()?;
    compiler.add_dependency("std::{env, path::PathBuf}")?;
    compiler.add_dependency("cbindgen::Config")?;
    compiler.add_newline()?;

    let func_context = FunctionContext::new("main", false, vec![], "");
    compiler.add_context(Context::Function(func_context))?;
    compiler.add_def_with_let(false, None, "cargo_manifest_dir",
        "env::var(\"CARGO_MANIFEST_DIR\").unwrap()",
    )?;
    compiler.add_def_with_let(false, None, "output_file", &format!(
        "PathBuf::from(&cargo_manifest_dir).join(\"{}.h\").display().to_string()",
        package_name,
    ))?;
    compiler.add_def_with_let(false, None, "config",
        "Config { language: cbindgen::Language::C, ..Default::default() }"
    )?;
    compiler.add_line(
        "cbindgen::generate_with_config(&cargo_manifest_dir, config) \
        .unwrap() \
        .write_to_file(&output_file);"
    )?;
    compiler.pop_context()?;
    compiler.flush(&package_folder.join("build.rs"))?;
    Ok(())
}

pub fn gen_cargo_toml(
    package_name: &str,
    package_folder: &Path,
    crates: &Vec<String>,
) -> Result<()> {
    let package_name_c = format!("{}-c", str::replace(package_name, "_", "-"));
    let package_name_rust = format!("{}_c", package_name);

    let mut compiler = SerializationCompiler::new();
    compiler.add_line("[package]")?;
    compiler.add_line(&format!("name = \"{}\"", package_name_c))?;
    compiler.add_line("version = \"0.1.0\"")?;
    compiler.add_line("edition = \"2021\"")?;
    compiler.add_newline()?;

    compiler.add_line("[lib]")?;
    compiler.add_line(&format!("name = \"{}\"", package_name_rust))?;
    compiler.add_line("path = \"src/lib.rs\"")?;
    compiler.add_line("crate-type = [\"cdylib\"]")?;
    compiler.add_newline()?;

    // Dependencies
    compiler.add_line("[dependencies]")?;
    for line in crates {
        compiler.add_line(line)?;
    }
    compiler.add_newline()?;

    // Build dependencies
    compiler.add_line("[build-dependencies]")?;
    compiler.add_line("cbindgen = \"0.23.0\"")?;
    compiler.add_newline()?;

    // Exclude the generated package from the workspace
    compiler.add_line("[workspace]")?;

    compiler.flush(&package_folder.join("Cargo.toml"))?;
    Ok(())
}
