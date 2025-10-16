fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "prost")]
    prost_build::compile_protos(
        &[
            "proto/mtid/stid.proto",
            "proto/mtid/dtid.proto",
            "proto/mtid/ttid.proto",
            "proto/mtid/qtid.proto",
        ],
        &["proto/"],
    )?;
    Ok(())
}
