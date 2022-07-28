use super::{
    compiler::{
        Context, FunctionArg, FunctionContext, SerializationCompiler, CArgInfo,
        MatchContext,
    },
    types::{ArgType, SelfArgType},
};
use color_eyre::eyre::{bail, Result};
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

pub fn add_extern_c_function(
    compiler: &mut SerializationCompiler,
    extern_name: Option<&str>,
    struct_ty: ArgType,
    func_name: &str,
    self_ty: Option<SelfArgType>,
    raw_args: Vec<(&str, ArgType)>,
    raw_ret: Option<ArgType>,
    use_error_code: bool,
) -> Result<()> {
    let (struct_name, struct_params) = match struct_ty {
        ArgType::Struct { ref name, ref params } => (name, params),
        _ => bail!("Expecting Struct argument type as struct_ty"),
    };

    let args = {
        let mut args = vec![];
        if self_ty.is_some() {
            args.push(FunctionArg::CSelfArg);
        }
        for (arg_name, arg_ty) in &raw_args {
            args.push(FunctionArg::CArg(CArgInfo::arg(arg_name, arg_ty.to_c_str())));
            if arg_ty.is_buffer() {
                args.push(FunctionArg::CArg(CArgInfo::len_arg(arg_name)));
            }
        }
        if let Some(ret_ty) = &raw_ret {
            args.push(FunctionArg::CArg(CArgInfo::ret_arg(ret_ty.to_c_str())));
            if ret_ty.is_buffer() {
                args.push(FunctionArg::CArg(CArgInfo::ret_len_arg()));
            }
        }
        args
    };

    let func_context = FunctionContext::new_extern_c(
        extern_name.unwrap_or(&format!("{}_{}", struct_name, func_name)),
        true, args, use_error_code,
    );
    compiler.add_context(Context::Function(func_context))?;

    // Format self argument
    if let Some(ref self_ty) = self_ty {
        let struct_name = struct_ty.to_rust_str();
        match self_ty {
            SelfArgType::Value => {
                compiler.add_unsafe_def_with_let(false, None, "self_",
                    &format!("Box::from_raw(self_ as *mut {})", struct_name))?;
            }
            SelfArgType::Ref => {
                compiler.add_unsafe_def_with_let(false, None, "self_box",
                    &format!("Box::from_raw(self_ as *mut {})", struct_name))?;
                compiler.add_unsafe_def_with_let(false, None, "self_",
                    "&**self_box")?;
            }
            SelfArgType::RefMut => {
                compiler.add_unsafe_def_with_let(false, None, "self_box",
                    &format!("Box::from_raw(self_ as *mut {})", struct_name))?;
                compiler.add_unsafe_def_with_let(false, None, "self_",
                    "&mut **self_box")?;
            }
            SelfArgType::Mut => {
                compiler.add_unsafe_def_with_let(true, None, "self_",
                    &format!("Box::from_raw(self_ as *mut {})", struct_name))?;
            }
        }
    }

    // Format arguments
    for (i, (arg_name, arg_ty)) in raw_args.iter().enumerate() {
        let left = format!("arg{}", i);
        let right = match arg_ty {
            ArgType::Primitive(_) => arg_name.to_string(),
            ArgType::Struct{..} => format!(
                "unsafe {{ *Box::from_raw({} as *mut {}) }}",
                arg_name, arg_ty.to_rust_str(),
            ),
            ArgType::Ref { ty } => format!(
                "unsafe {{ Box::from_raw({} as *mut {}) }}",
                arg_name, ty.to_rust_str(),
            ),
            ArgType::RefMut { ty } => format!(
                "{} as *mut {}",
                arg_name, ty.to_rust_str(),
            ),
            ArgType::Buffer => format!(
                "unsafe {{ std::slice::from_raw_parts({}, {}_len) }}",
                arg_name, arg_name,
            ),
        };
        compiler.add_def_with_let(false, None, &left, &right)?;
    }

    // Generate function arguments and return type
    let args = raw_args.iter()
        .enumerate()
        .map(|(i, (_, arg_ty))| match arg_ty {
            ArgType::Ref{..} => format!("&arg{}", i),
            ArgType::RefMut{..} => format!("unsafe {{ &mut *arg{} }}", i),
            _ => format!("arg{}", i),
        })
        .collect::<Vec<_>>();
    let ret_ty = if let Some(ref ret_ty) = raw_ret {
        match ret_ty {
            ArgType::Ref { ty } => Some(format!("*const {}", &ty.to_rust_str())),
            ArgType::RefMut { ty } => Some(format!("*mut {}", &ty.to_rust_str())),
            _ => None,
        }
    } else {
        None
    };

    // Call function wrapper
    let (caller, func) = if self_ty.is_some() {
        (Some("self_".to_string()), func_name.to_string())
    } else {
        (None, format!(
            "{}::<{}>::{}",
            struct_name,
            struct_params.iter()
                .map(|p| p.to_rust_str()).collect::<Vec<_>>().join(", "),
            func_name,
        ))
    };
    if use_error_code || raw_ret.is_some() {
        compiler.add_func_call_with_let("value", ret_ty, caller, &func, args, false)?;
    } else {
        compiler.add_func_call(caller, &func, args, false)?;
    }

    // Unwrap result if uses an error code
    if use_error_code {
        let match_context = if raw_ret.is_some() {
            MatchContext::new_with_def(
                "value",
                vec!["Ok(value)".to_string(), "Err(_)".to_string()],
                "value",
            )
        } else {
            MatchContext::new(
                "value",
                vec!["Ok(_)".to_string(), "Err(_)".to_string()],
            )
        };
        compiler.add_context(Context::Match(match_context))?;
        if raw_ret.is_some() {
            compiler.add_return_val("value", false)?;
        } else {
            compiler.add_return_val("", false)?;
        }
        compiler.pop_context()?;
        compiler.add_return_val("1", true)?;
        compiler.pop_context()?;
    }

    // Marshall return value into C type
    if let Some(ret_ty) = &raw_ret {
        match ret_ty {
            ArgType::Primitive(_) => {
                compiler.add_unsafe_set("return_ptr", "value")?;
            }
            ArgType::Struct{..} => {
                compiler.add_func_call_with_let("value", None, None,
                   "Box::into_raw", vec!["Box::new(value)".to_string()],
                   false)?;
                compiler.add_unsafe_set("return_ptr", "value as _")?;
            }
            ArgType::Ref{..} | ArgType::RefMut{..} => {
                compiler.add_unsafe_set("return_ptr", "value as _")?;
            },
            ArgType::Buffer => unimplemented!(),
        }
    }

    // Unformat arguments
    if let Some(ref self_ty) = self_ty {
        let arg_name = if self_ty.is_ref() {
            "self_box"
        } else {
            "self_"
        }.to_string();
        compiler.add_func_call(None, "Box::into_raw", vec![arg_name], false)?;
    }
    for (i, (_, arg_ty)) in raw_args.iter().enumerate() {
        match arg_ty {
            ArgType::Primitive(_) => { continue; },
            ArgType::Struct{..} => { continue; },
            ArgType::Ref{..} => {
                compiler.add_func_call(None, "Box::into_raw", vec![format!("arg{}", i)], false)?;
            },
            ArgType::RefMut{..} => { continue; },
            ArgType::Buffer => { continue; },
        };
    }

    if use_error_code {
        compiler.add_line("0")?;
    }

    compiler.pop_context()?; // end of function
    compiler.add_newline()?;
    Ok(())
}
