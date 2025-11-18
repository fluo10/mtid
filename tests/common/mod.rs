use caretta_id::*;

#[cfg(feature = "std")]
fn assert_string_convertion(value: CarettaId) {
    assert_eq!(value, value.to_string().parse::<CarettaId>().unwrap());
}

fn assert_integer_conversion(value: CarettaId) {
    assert_eq!(value, <CarettaId>::try_from(<u64>::from(value)).unwrap());
}

fn assert_le_bytes_conversion(value: CarettaId) {
    let bytes = value.to_le_bytes();
    assert_eq!(value, CarettaId::from_le_bytes(bytes).unwrap());
    assert_eq!(value, CarettaId::from_le_bytes_lossy(bytes));
}
fn assert_be_bytes_conversion(value: CarettaId) {
    let bytes = value.to_be_bytes();
    assert_eq!(value, CarettaId::from_be_bytes(bytes).unwrap());
    assert_eq!(value, CarettaId::from_be_bytes_lossy(bytes));
}
fn assert_be_bytes_compact_conversion(value: CarettaId) {
    let bytes = value.to_be_bytes_compact();
    assert_eq!(value, CarettaId::from_be_bytes_compact(bytes).unwrap());
    assert_eq!(value, CarettaId::from_be_bytes_compact_lossy(bytes));
}
fn assert_le_bytes_compact_conversion(value: CarettaId) {
    let bytes = value.to_le_bytes_compact();
    assert_eq!(value, CarettaId::from_le_bytes_compact(bytes).unwrap());
    assert_eq!(value, CarettaId::from_le_bytes_compact_lossy(bytes));
}

pub fn assert_conversion(value: CarettaId) {
    #[cfg(feature = "std")]
    assert_string_convertion(value);
    assert_integer_conversion(value);
    assert_be_bytes_conversion(value);
    assert_be_bytes_compact_conversion(value);
    assert_le_bytes_conversion(value);
    assert_le_bytes_compact_conversion(value);
}
