fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature="prost")]
    prost_build::compile_protos(
        &[
            "proto/tripod_id/stid.proto",
            "proto/tripod_id/dtid.proto",
            "proto/tripod_id/ttid.proto"
        ],
        &["proto/"]
    )?;
    Ok(())
}