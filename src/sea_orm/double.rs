use crate::Double;

impl From<Double> for sea_orm::Value {
    fn from(value: Double) -> Self {
        sea_orm::sea_query::Value::Unsigned(Some(value.into()))
    }
}

impl sea_orm::TryGetable for Double {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
        match <u32 as sea_orm::TryGetable>::try_get_by(res, index) {
            Ok(x) => Double::try_from(x).map_err(|e| sea_orm::TryGetError::DbErr(sea_orm::DbErr::TryIntoErr { from: stringify!(u32), into: stringify!(tripod_id::Double), source: Box::new(e) })),
            Err(x) => Err(x)
        }
    }
}

impl sea_orm::sea_query::ValueType for Double {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <u32 as sea_orm::sea_query::ValueType>::try_from(v).map(|x| <Double as TryFrom<u32>>::try_from(x).map_err(|_| sea_orm::sea_query::ValueTypeErr ))?
    }
    fn type_name() -> String {
        stringify!(tripod_id::Double).to_owned()
    }
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Unsigned
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::Unsigned
    }
}

impl sea_orm::sea_query::Nullable for Double {
    fn null() -> sea_orm::Value {
        <u32 as sea_orm::sea_query::Nullable>::null()
    }
}