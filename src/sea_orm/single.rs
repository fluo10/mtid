use crate::Single;

impl From<Single> for sea_orm::Value {
    fn from(value: Single) -> Self {
        sea_orm::sea_query::Value::SmallUnsigned(Some(value.into()))
    }
}

impl sea_orm::TryGetable for Single {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
        match <u16 as sea_orm::TryGetable>::try_get_by(res, index) {
            Ok(x) => Single::try_from(x).map_err(|e| sea_orm::TryGetError::DbErr(sea_orm::DbErr::TryIntoErr { from: "u16", into: "tripod_id::Single", source: Box::new(e) })),
            Err(x) => Err(x)
        }
    }
}

impl sea_orm::sea_query::ValueType for Single {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <u16 as sea_orm::sea_query::ValueType>::try_from(v).map(|x| <Single as TryFrom<u16>>::try_from(x).map_err(|_| sea_orm::sea_query::ValueTypeErr ))?
    }
    fn type_name() -> String {
        stringify!(tripod_id::Single).to_owned()
    }
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::SmallUnsigned
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::SmallUnsigned
    }
}

impl sea_orm::sea_query::Nullable for Single {
    fn null() -> sea_orm::Value {
        <u16 as sea_orm::sea_query::Nullable>::null()
    }
}