use super::CarettaIdQ;

impl From<CarettaIdQ> for sea_orm::Value {
    fn from(value: CarettaIdQ) -> Self {
        sea_orm::sea_query::Value::BigUnsigned(Some(value.into()))
    }
}

impl sea_orm::TryGetable for CarettaIdQ {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        match <u64 as sea_orm::TryGetable>::try_get_by(res, index) {
            Ok(x) => CarettaIdQ::try_from(x).map_err(|e| {
                sea_orm::TryGetError::DbErr(sea_orm::DbErr::TryIntoErr {
                    from: stringify!(u64),
                    into: stringify!(CarettaIdQ),
                    source: Box::new(e),
                })
            }),
            Err(x) => Err(x),
        }
    }
}

impl sea_orm::sea_query::ValueType for CarettaIdQ {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <u64 as sea_orm::sea_query::ValueType>::try_from(v).map(|x| {
            <CarettaIdQ as TryFrom<u64>>::try_from(x).map_err(|_| sea_orm::sea_query::ValueTypeErr)
        })?
    }
    fn type_name() -> String {
        stringify!(CarettaIdQ).to_owned()
    }
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::BigUnsigned
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::BigUnsigned
    }
}

impl sea_orm::sea_query::Nullable for CarettaIdQ {
    fn null() -> sea_orm::Value {
        <u64 as sea_orm::sea_query::Nullable>::null()
    }
}
