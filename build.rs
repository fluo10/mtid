fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "prost")]
    prost_build::compile_protos(
        &[
            "mtid-proto/mtid/stid.proto",
            "mtid-proto/mtid/dtid.proto",
            "mtid-proto/mtid/ttid.proto",
            "mtid-proto/mtid/qtid.proto",
        ],
        &["mtid-proto/"],
    )?;
    Ok(())
}
