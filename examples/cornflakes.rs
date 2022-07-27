use ffiber::CDylibCompiler;
use ffiber::types::{ArgType, SelfArgType};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let mut compiler = CDylibCompiler::new("mlx5-datapath", ".");
    compiler.add_crate("bumpalo", vec![
        ("git", "\"https://github.com/deeptir18/bumpalo\""),
        ("features", "[\"collections\"]"),
    ])?;
    compiler.add_crate("mlx5-datapath",
        vec![("path", "\"/users/ygina/cornflakes/mlx5-datapath\"")])?;
    compiler.add_crate("cornflakes-libos",
        vec![("path", "\"/users/ygina/cornflakes/cornflakes-libos\"")])?;
    compiler.add_dependency("bumpalo::Bump")?;
    compiler.add_dependency(
        "mlx5_datapath::datapath::connection::Mlx5Connection")?;
    compiler.add_dependency("cornflakes_libos::datapath::{{Datapath, \
        ReceivedPkt}}")?;

    // Bump
    compiler.add_extern_c_function(
        None,
        "Bump",
        "reset",
        Some(SelfArgType::Mut),
        vec![],
        None,
        false,
    )?;

    // ReceivedPkt
    compiler.add_extern_c_function(
        Some("ReceivedPkt_msg_id"),
        "ReceivedPkt<Mlx5Connection>",
        "msg_id",
        Some(SelfArgType::Value),
        vec![],
        Some(ArgType::Primitive("u32".to_string())),
        false,
    )?;
    compiler.add_extern_c_function(
        Some("ReceivedPkt_conn_id"),
        "ReceivedPkt<Mlx5Connection>",
        "conn_id",
        Some(SelfArgType::Value),
        vec![],
        Some(ArgType::Primitive("usize".to_string())),
        false,
    )?;

    // Mlx5Connection
    compiler.add_extern_c_function(
        None,
        "Mlx5Connection",
        "set_copying_threshold",
        Some(SelfArgType::Mut),
        vec![
            ("copying_threshold", ArgType::Primitive("usize".to_string())),
        ],
        None,
        false,
    )?;
    compiler.add_extern_c_function(
        None,
        "Mlx5Connection",
        "add_memory_pool",
        Some(SelfArgType::Mut),
        vec![
            ("buf_size", ArgType::Primitive("usize".to_string())),
            ("min_elts", ArgType::Primitive("usize".to_string())),
        ],
        None,
        true,
    )?;
    compiler.add_extern_c_function(
        None,
        "Mlx5Connection",
        "add_tx_mempool",
        Some(SelfArgType::Mut),
        vec![
            ("size", ArgType::Primitive("usize".to_string())),
            ("min_elts", ArgType::Primitive("usize".to_string())),
        ],
        None,
        true,
    )?;

    compiler.flush()?;
    Ok(())
}
