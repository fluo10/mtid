fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(
        &["proto/mtid_example.proto"],
        &["proto", "../../mtid-proto/"],
    )?;

    Ok(())
}
