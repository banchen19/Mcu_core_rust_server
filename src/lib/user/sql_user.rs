use sqlx::Row;

use crate::lib::config::ConnectionType;

use super::web_user::RegisterUser;

// 创建用户表
pub async fn create_user_table(conn: ConnectionType) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql).execute(&mut conn).await?;
            Ok(ConnectionType::Sqlite(conn))
        }

        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql).execute(&mut conn).await?;
            Ok(ConnectionType::Mysql(conn))
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql).execute(&mut conn).await?;
            Ok(ConnectionType::Postgres(conn))
        }
    }
}

// 注册账号
pub async fn register_user(conn: ConnectionType, user: &RegisterUser) -> Result<u64, sqlx::Error> {
    let sql = r#"
        INSERT INTO user (email, password) VALUES (?, ?);
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_rowid().try_into().unwrap())
        }
        ConnectionType::Mysql(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_id())
        }
        ConnectionType::Postgres(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.rows_affected())
        }
    }
}

// 登录账号
pub async fn login_user(conn: ConnectionType, user: &RegisterUser) -> Result<i64, sqlx::Error> {
    let sql = r#"
        SELECT id FROM user WHERE email = ? AND password = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let row = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .fetch_one(&mut conn)
                .await?;

            let id: i64 = row.try_get("id").unwrap();
            Ok(id)
        }
        ConnectionType::Mysql(mut conn) => {
            let row = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .fetch_one(&mut conn)
                .await?;
            let id: i64 = row.try_get("id").unwrap();
            Ok(id)
        }
        ConnectionType::Postgres(mut conn) => {
            let row = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .fetch_one(&mut conn)
                .await?;
            let id: i64 = row.try_get("id").unwrap();
            Ok(id)
        }
    }
}

// 测试
#[tokio::test]
async fn test_register_user() {
    use crate::lib::config::HttpServerConfig;
    let conn = crate::lib::config::init_db(&HttpServerConfig::default()).await;
    let user = RegisterUser {
        email: " [email protected]".to_string(),
        password: "123456".to_string(),
    };
    println!("{}", register_user(conn, &user).await.unwrap());
}
