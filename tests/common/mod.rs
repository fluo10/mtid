use caretta_id::*;

pub fn assert_string_convertion(value: CarettaId) {
    assert_eq!(value, value.to_string().parse::<CarettaId>().unwrap());
}

pub fn assert_integer_conversion(value: CarettaId) {
    assert_eq!(value, <CarettaId>::try_from(<u64>::from(value)).unwrap());
}
