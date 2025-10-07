use crate::Stid;

impl From<Stid> for sea_orm::Value {
    fn from(value: Stid) -> Self {
        sea_orm::sea_query::Value::SmallUnsigned(Some(value.into()))
    }
}

impl sea_orm::TryGetable for Stid {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
        match <u16 as sea_orm::TryGetable>::try_get_by(res, index) {
            Ok(x) => Stid::try_from(x).map_err(|e| sea_orm::TryGetError::DbErr(sea_orm::DbErr::TryIntoErr { from: "u16", into: "tripod_id::Stid", source: Box::new(e) })),
            Err(x) => Err(x)
        }
    }
}

impl sea_orm::sea_query::ValueType for Stid {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <u16 as sea_orm::sea_query::ValueType>::try_from(v).map(|x| <Stid as TryFrom<u16>>::try_from(x).map_err(|_| sea_orm::sea_query::ValueTypeErr ))?
    }
    fn type_name() -> String {
        stringify!(Stid).to_owned()
    }
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::SmallUnsigned
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::SmallUnsigned
    }
}

impl sea_orm::sea_query::Nullable for Stid {
    fn null() -> sea_orm::Value {
        <u16 as sea_orm::sea_query::Nullable>::null()
    }
}