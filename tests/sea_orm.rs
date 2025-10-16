#![cfg(feature = "sea-orm")]

use std::u32;

use mtid::{Dtid, Stid, Ttid};
use rand::Rng;
use sea_orm::{
    DatabaseBackend, MockDatabase, MockExecResult, Transaction,
    entity::{prelude::*, *},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "mtids")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub single: Stid,
    pub double: Dtid,
    pub triple: Ttid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

async fn assert_model(model: Model) {
    let active_model = model.clone().into_active_model().reset_all();

    let db = MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results([vec![model.clone()], vec![model.clone()]])
        .append_exec_results([MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    assert_eq!(active_model.insert(&db).await.unwrap(), model.clone());
    assert_eq!(Entity::find().one(&db).await.unwrap(), Some(model.clone()));
    assert_eq!(
        db.into_transaction_log(),
        [
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"INSERT INTO "mtids" ("id", "single", "double", "triple") VALUES (?, ?, ?, ?)"#,
                [
                    model.id.into(),
                    model.single.into(),
                    model.double.into(),
                    model.triple.into()
                ]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"SELECT "mtids"."id", "mtids"."single", "mtids"."double", "mtids"."triple" FROM "mtids" WHERE "mtids"."id" = ? LIMIT ?"#,
                [1u32.into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"SELECT "mtids"."id", "mtids"."single", "mtids"."double", "mtids"."triple" FROM "mtids" LIMIT ?"#,
                [1u64.into()]
            )
        ]
    );
}

#[tokio::test]
async fn nil() {
    assert_model(Model {
        id: 1,
        single: Stid::NIL,
        double: Dtid::NIL,
        triple: Ttid::NIL,
    })
    .await;
}

#[tokio::test]
async fn max() {
    assert_model(Model {
        id: u32::MAX,
        single: Stid::MAX,
        double: Dtid::MAX,
        triple: Ttid::MAX,
    })
    .await;
}

#[tokio::test]
async fn random() {
    let mut rng = rand::rng();
    for _ in 0..10 {
        assert_model(Model {
            id: rng.random(),
            single: rng.random(),
            double: rng.random(),
            triple: rng.random(),
        })
        .await;
    }
}
