use ffiber::CDylibCompiler;
use ffiber::types::{ArgType, SelfArgType};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let mut compiler = CDylibCompiler::new("mlx5-datapath", ".");
    compiler.add_crate_path("mlx5-datapath",
        "/users/ygina/cornflakes/mlx5-datapath");
    /*
    compiler.add_dependency(
        "mlx5_datapath::datapath::connection::Mlx5Connection")?;
    compiler.add_extern_c_function(
        None,
        "Mlx5Connection",
        "set_copying_threshold",
        Some(SelfArgType::Mut),
        vec![
            ("copying_threshold", ArgType::Primitive { ty: "usize".to_string() }),
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
            ("buf_size", ArgType::Primitive { ty: "usize".to_string() }),
            ("min_elts", ArgType::Primitive { ty: "usize".to_string() }),
        ],
        None,
        false,
    )?;
    compiler.add_extern_c_function(
        None,
        "Mlx5Connection",
        "add_tx_mempool",
        Some(SelfArgType::Mut),
        vec![
            ("size", ArgType::Primitive { ty: "usize".to_string() }),
            ("min_elts", ArgType::Primitive { ty: "usize".to_string() }),
        ],
        None,
        false,
    )?;
    */
    compiler.flush()?;
    Ok(())
}
