use ffiber::CDylibCompiler;
use ffiber::types::{Type, SelfType};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let mut compiler = CDylibCompiler::new_with_output_folder(
        "mlx5-datapath", ".");
    compiler.add_dependency("bumpalo", vec![
        ("git", "\"https://github.com/deeptir18/bumpalo\""),
        ("features", "[\"collections\"]"),
    ])?;
    compiler.add_dependency("mlx5-datapath",
        vec![("path", "\"/users/ygina/cornflakes/mlx5-datapath\"")])?;
    compiler.add_dependency("cornflakes-libos",
        vec![("path", "\"/users/ygina/cornflakes/cornflakes-libos\"")])?;
    compiler.import("bumpalo::Bump")?;
    compiler.import(
        "mlx5_datapath::datapath::connection::Mlx5Connection")?;
    compiler.import("cornflakes_libos::datapath::{{Datapath, \
        ReceivedPkt}}")?;

    // Bump
    compiler.add_extern_c_function(
        Type::new_struct("Bump"),
        SelfType::Mut,
        "reset",
        vec![],
        None,
        false,
    )?;

    // ReceivedPkt
    let struct_ty = Type::Struct {
        name: "ReceivedPkt".to_string(),
        params: vec![Box::new(Type::new_struct("Mlx5Connection"))],
    };
    compiler.add_extern_c_function(
        struct_ty.clone(),
        SelfType::Value,
        "msg_id",
        vec![],
        Some(Type::Primitive("u32".to_string())),
        false,
    )?;
    compiler.add_extern_c_function(
        struct_ty.clone(),
        SelfType::Value,
        "conn_id",
        vec![],
        Some(Type::Primitive("usize".to_string())),
        false,
    )?;

    // Mlx5Connection
    compiler.add_extern_c_function(
        Type::new_struct("Mlx5Connection"),
        SelfType::Mut,
        "set_copying_threshold",
        vec![
            ("copying_threshold", Type::Primitive("usize".to_string())),
        ],
        None,
        false,
    )?;
    compiler.add_extern_c_function(
        Type::new_struct("Mlx5Connection"),
        SelfType::Mut,
        "add_memory_pool",
        vec![
            ("buf_size", Type::Primitive("usize".to_string())),
            ("min_elts", Type::Primitive("usize".to_string())),
        ],
        None,
        true,
    )?;
    compiler.add_extern_c_function(
        Type::new_struct("Mlx5Connection"),
        SelfType::Mut,
        "add_tx_mempool",
        vec![
            ("size", Type::Primitive("usize".to_string())),
            ("min_elts", Type::Primitive("usize".to_string())),
        ],
        None,
        true,
    )?;

    compiler.flush()?;
    Ok(())
}
