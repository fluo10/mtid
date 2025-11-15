#![cfg(feature = "prost")]

use caretta_id::CarettaIdProto;

macro_rules! impl_mod {
    ($mod_name:ident,$Self:ty, $Proto:ty, $Uint:ty) => {
        mod $mod_name {
            #[test]
            fn nil() {
                let nil = <$Proto>::from(0);
                assert_eq!(<$Self>::NIL, <$Self>::try_from(nil).unwrap());
            }

            #[test]
            fn max() {
                let max = <$Proto>::from(<$Uint>::from(<$Self>::CAPACITY) - 1);
                assert_eq!(<$Self>::MAX, <$Self>::try_from(max).unwrap());
            }

            #[test]
            #[should_panic]
            fn oversized() {
                let oversized = <$Proto>::from(<$Uint>::from(<$Self>::CAPACITY));
                let _ = <$Self>::try_from(oversized).unwrap();
            }
        }
    };
}

impl_mod!(
    caretta_id_s,
    caretta_id::CarettaIdS,
    caretta_id::proto::CarettaIdS,
    u32
);
impl_mod!(
    caretta_id_d,
    caretta_id::CarettaIdD,
    caretta_id::proto::CarettaIdD,
    u32
);
impl_mod!(
    caretta_id_t,
    caretta_id::CarettaIdT,
    caretta_id::proto::CarettaIdT,
    u64
);
impl_mod!(
    caretta_id_q,
    caretta_id::CarettaIdQ,
    caretta_id::proto::CarettaIdQ,
    u64
);
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
        value: <u64>::from(<caretta_id::CarettaId>::CAPACITY) - 1,
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
        value: <u64>::from(<caretta_id::CarettaId>::CAPACITY),
    };
    let _ = <caretta_id::CarettaId>::try_from(oversized).unwrap();
}
