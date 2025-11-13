fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "prost")]
    prost_build::compile_protos(
        &[
            "caretta-id-proto/caretta_id/caretta_id_s.proto",
            "caretta-id-proto/caretta_id/caretta_id_d.proto",
            "caretta-id-proto/caretta_id/caretta_id_t.proto",
            "caretta-id-proto/caretta_id/caretta_id_q.proto",
        ],
        &["caretta-id-proto/"],
    )?;
    Ok(())
}
