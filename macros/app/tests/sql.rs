#![cfg(feature = "sql")]
use neko_cat_macros::Sql;
use tokio_postgres::{Client, NoTls};

#[derive(Sql)]
struct CUser {
    #[opt(sql = "TEXT PRIMARY KEY")]
    uuid: &'static str,
    #[opt(sql = "TEXT NOT NULL")]
    name: String,
    #[opt(sql = "TEXT NOT NULL")]
    nick: &'static str,
}

async fn setup_db() -> Client {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=12345 dbname=postgres",
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    CUser::up_migrations_temporary_table_if_not_exists(&client)
        .await
        .expect("up_migrations_temporary_table_if_not_exists failed");

    client
}

#[tokio::test]
async fn sql_basic() {
    let client = setup_db().await;

    let user = CUser {
        uuid: "2",
        name: "John".to_string(),
        nick: "Doe",
    };

    let insert_res = user.sql_insert(&client).await.expect("Insert failed");
    assert_eq!(insert_res, 1);

    let rows = user
        .sql_select_all(&client)
        .await
        .expect("Select all failed");
    assert_eq!(rows.len(), 1);

    let row = &rows[0];
    assert_eq!(row.get::<_, &str>("uuid"), "2");
    assert_eq!(row.get::<_, &str>("name"), "John");
    assert_eq!(row.get::<_, &str>("nick"), "Doe");
}

#[tokio::test]
async fn sql_per_field_helpers() {
    let client = setup_db().await;

    let user = CUser {
        uuid: "1",
        name: "John".to_string(),
        nick: "Doe",
    };

    user.sql_insert(&client).await.expect("Insert failed");

    let rows = user
        .sql_select_by_name(&client)
        .await
        .expect("Select by name failed");
    assert_eq!(rows.len(), 1);

    let deleted = user
        .sql_delete_by_name(&client)
        .await
        .expect("Delete by name failed");
    assert_eq!(deleted, 1);

    user.sql_insert(&client).await.expect("Insert failed");

    let updated = user
        .sql_update_by_name(&client)
        .await
        .expect("Update by name failed");
    assert_eq!(updated, 1);
}
