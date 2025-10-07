#![cfg(feature = "sea-orm")]

use std::u32;

use tripod_id::{Stid, Dtid, Ttid};
use rand::Rng;
use sea_orm::{
    entity::{prelude::*, *}, DatabaseBackend, MockDatabase, MockExecResult, Transaction
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tripod_ids")]
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
        .append_query_results([
            vec![model.clone()],
            vec![model.clone()],
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }
        ]).into_connection();

    assert_eq!(
        active_model.insert(&db).await.unwrap(),
        model.clone()
    );
    assert_eq!(Entity::find().one(&db).await.unwrap(), Some(model.clone()));
    assert_eq!(
        db.into_transaction_log(), 
        [
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"INSERT INTO "tripod_ids" ("id", "single", "double", "triple") VALUES (?, ?, ?, ?)"#,
                [model.id.into(), model.single.into(), model.double.into(), model.triple.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"SELECT "tripod_ids"."id", "tripod_ids"."single", "tripod_ids"."double", "tripod_ids"."triple" FROM "tripod_ids" WHERE "tripod_ids"."id" = ? LIMIT ?"#,
                [1u32.into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"SELECT "tripod_ids"."id", "tripod_ids"."single", "tripod_ids"."double", "tripod_ids"."triple" FROM "tripod_ids" LIMIT ?"#,
                [1u64.into()]
            )
        ]
    );

}

#[tokio::test]
async fn nil() {
    assert_model(Model{
        id: 1,
        single: Stid::NIL,
        double: Dtid::NIL,
        triple: Ttid::NIL
    }).await;
}

#[tokio::test]
async fn max() {
    assert_model(Model {
        id: u32::MAX,
        single: Stid::MAX,
        double: Dtid::MAX,
        triple: Ttid::MAX
    }).await;
}

#[tokio::test]
async fn random() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        assert_model(Model {
            id: rng.r#gen(),
            single: rng.r#gen(),
            double: rng.r#gen(),
            triple: rng.r#gen()
        }).await;
    }
}
