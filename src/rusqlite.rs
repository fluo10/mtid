use super::CarettaId;
use rusqlite::{ToSql, types::FromSql};

impl FromSql for CarettaId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let int = u64::column_result(value)?;
        Ok(Self::from_u64_lossy(int))
    }
}

impl ToSql for CarettaId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.as_u64().to_sql()
    }
}
