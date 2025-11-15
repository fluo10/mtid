use sea_orm::TryFromU64;

use super::CarettaId;

impl From<CarettaId> for sea_orm::Value {
    fn from(value: CarettaId) -> Self {
        sea_orm::sea_query::Value::BigUnsigned(Some(value.into()))
    }
}

impl sea_orm::TryGetable for CarettaId {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        <u64 as sea_orm::TryGetable>::try_get_by(res, index).map(|x| CarettaId::from_u64_lossy(x))
    }
}

impl sea_orm::sea_query::ValueType for CarettaId {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <u64 as sea_orm::sea_query::ValueType>::try_from(v).map(|x| {
            CarettaId::from_u64_lossy(x)
        })
    }
    fn type_name() -> String {
        stringify!(CarettaId).to_owned()
    }
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::BigUnsigned
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::BigUnsigned
    }
}

impl sea_orm::sea_query::Nullable for CarettaId {
    fn null() -> sea_orm::Value {
        <u64 as sea_orm::sea_query::Nullable>::null()
    }
}

impl TryFromU64 for CarettaId {
    fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
        Self::try_from(n).map_err(|x| sea_orm::DbErr::TryIntoErr { from: stringify!(u64), into: stringify!(CarettaId), source: Box::new(x) })
    }
}