fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .extern_path(".caretta_id", "::caretta_id::proto")
        .compile_protos(
            &["proto/caretta_id_example.proto"],
            &["proto", "../../caretta-id-proto/"],
        )?;

    Ok(())
}
