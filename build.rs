fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "prost")]
    prost_build::Config::new()
    .type_attribute("caretta_id.CarettaIdS", r#"#[deprecated(since = "0.8.0", note = "The caretta-id has been renewed. Use new CarettaId instead")]"#)
    .type_attribute("caretta_id.CarettaIdD", r#"#[deprecated(since = "0.8.0", note = "The caretta-id has been renewed. Use new CarettaId instead")]"#)
    .type_attribute("caretta_id.CarettaIdT", r#"#[deprecated(since = "0.8.0", note = "The caretta-id has been renewed. Use new CarettaId instead")]"#)
    .type_attribute("caretta_id.CarettaIdQ", r#"#[deprecated(since = "0.8.0", note = "The caretta-id has been renewed. Use new CarettaId instead")]"#)
    .compile_protos(
        &[
            "caretta-id-proto/caretta_id/caretta_id_s.proto",
            "caretta-id-proto/caretta_id/caretta_id_d.proto",
            "caretta-id-proto/caretta_id/caretta_id_t.proto",
            "caretta-id-proto/caretta_id/caretta_id_q.proto",
            "caretta-id-proto/caretta_id/caretta_id.proto",
        ],
        &["caretta-id-proto/"],
    )?;
    Ok(())
}
