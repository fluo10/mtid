#![cfg(feature = "prost")]

use caretta_id::CarettaIdProto;

#[test]
fn nil() {
    let nil = CarettaIdProto { value: 0 };
    assert_eq!(
        <caretta_id::CarettaId>::NIL,
        <caretta_id::CarettaId>::try_from(nil).unwrap()
    );
}

#[test]
fn max() {
    let max = CarettaIdProto {
        value: <u64>::from(<caretta_id::CarettaId>::MAX),
    };
    assert_eq!(
        <caretta_id::CarettaId>::MAX,
        <caretta_id::CarettaId>::try_from(max).unwrap()
    );
}

#[test]
#[should_panic]
fn oversized() {
    let oversized = CarettaIdProto {
        value: <u64>::from(<caretta_id::CarettaId>::MAX) + 1,
    };
    let _ = <caretta_id::CarettaId>::try_from(oversized).unwrap();
}
