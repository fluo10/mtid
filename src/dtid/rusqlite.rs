use super::Dtid;
use rusqlite::{Error, ToSql, types::FromSql};

impl FromSql for Dtid {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let int = u32::column_result(value)?;
        Self::try_from(int).or_else(|e| Err(rusqlite::types::FromSqlError::Other(Box::new(e))))
    }
}

impl ToSql for Dtid {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(u32::from(*self).into())
    }
}
