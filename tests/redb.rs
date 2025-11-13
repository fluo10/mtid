#![cfg(feature = "redb")]

use std::sync::LazyLock;

use redb::{Database, backends::InMemoryBackend};

const DATABASE: LazyLock<redb::Database> = LazyLock::new(|| {
    Database::builder()
        .create_with_backend(InMemoryBackend::new())
        .unwrap()
});

macro_rules! redb_test_mod {
    ($mod_name:ident, $SelfT:ty) => {
        mod $mod_name {
            use redb::*;
            const TABLE: TableDefinition<$SelfT, $SelfT> =
                TableDefinition::new(stringify!($mod_name));

            fn assert_insert(key: $SelfT, value: $SelfT) {
                let database = super::DATABASE;
                {
                    let write_txn = database.begin_write().unwrap();
                    {
                        let mut table = write_txn.open_table(TABLE).unwrap();
                        let _ = table.insert(&key, &value).unwrap();
                    }
                    write_txn.commit().unwrap();
                }
                {
                    let read_txn = database.begin_read().unwrap();
                    {
                        let table = read_txn.open_table(TABLE).unwrap();
                        assert_eq!(table.get(key).unwrap().unwrap().value(), value);
                    }
                }
            }
            #[test]
            fn nil() {
                assert_insert(<$SelfT>::NIL, <$SelfT>::random());
            }
            #[test]
            fn max() {
                assert_insert(<$SelfT>::MAX, <$SelfT>::random());
            }
            #[test]
            fn random() {
                for _ in 0..10 {
                    assert_insert(<$SelfT>::random(), <$SelfT>::random());
                }
            }
        }
    };
}

redb_test_mod!(stid, mtid::Stid);
redb_test_mod!(dtid, mtid::Dtid);
redb_test_mod!(ttid, mtid::Ttid);
redb_test_mod!(qtid, mtid::Qtid);
