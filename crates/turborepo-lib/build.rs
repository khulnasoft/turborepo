fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["src/daemon/proto/turbod.proto"], &["src/daemon/proto"])?;

    let capnpc_result = capnpc::CompilerCommand::new()
        .file("./src/hash/proto.capnp")
        .default_parent_module(vec!["hash".to_string()])
        .run();

    let invocation = std::env::var("RUSTC_WRAPPER").unwrap_or_default();
    if invocation.ends_with("rust-analyzer") {
        if capnpc_result.is_err() {
            println!("cargo:warning=capnpc failed, but continuing with rust-analyzer");
        }

        return Ok(());
    } else {
        capnpc_result.expect("schema compiler command");
    }

    Ok(())
}
