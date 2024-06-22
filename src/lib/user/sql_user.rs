use futures_util::TryStreamExt;
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

            Ok(row.try_get("id")?)
        }
        ConnectionType::Mysql(mut conn) => {
            let row = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .fetch_one(&mut conn)
                .await?;
            Ok(row.try_get("id")?)
        }
        ConnectionType::Postgres(mut conn) => {
            let row = sqlx::query(sql)
                .bind(&user.email)
                .bind(&user.password)
                .fetch_one(&mut conn)
                .await?;
            Ok(row.try_get("id")?)
        }
    }
}

// 修改密码
pub async fn change_password(
    conn: ConnectionType,
    user: &RegisterUser,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        UPDATE user SET password = ? WHERE email = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&user.password)
                .bind(&user.email)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_rowid().try_into().unwrap())
        }
        ConnectionType::Mysql(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&user.password)
                .bind(&user.email)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_id())
        }
        ConnectionType::Postgres(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&user.password)
                .bind(&user.email)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.rows_affected())
        }
    }
}

// admin-获取所有用户
pub async fn get_all_user(conn: ConnectionType) -> Result<Vec<RegisterUser>, sqlx::Error> {
    let sql = r#"
        SELECT email, password FROM user;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let mut vec = Vec::new();
            let mut quer = sqlx::query(sql).fetch(&mut conn);
            while let Some(row) = quer.try_next().await? {
                vec.push(RegisterUser {
                    email: row.try_get("email")?,
                    password: "********".to_string(),
                });
            }
            Ok(vec)
        }
        ConnectionType::Mysql(mut conn) => {
            let mut vec = Vec::new();
            let mut quer = sqlx::query(sql).fetch(&mut conn);
            while let Some(row) = quer.try_next().await? {
                vec.push(RegisterUser {
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                });
            }
            Ok(vec)
        }
        ConnectionType::Postgres(mut conn) => {
            let mut vec = Vec::new();
            let mut quer = sqlx::query(sql).fetch(&mut conn);
            while let Some(row) = quer.try_next().await? {
                vec.push(RegisterUser {
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                });
            }
            Ok(vec)
        }
    }
}

// 获取用户uid
pub async fn get_user_id(conn: ConnectionType, email: &str) -> Result<i64, sqlx::Error> {
    let sql = r#"
        SELECT id FROM user WHERE email = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let row = sqlx::query(sql).bind(email).fetch_one(&mut conn).await?;
            Ok(row.try_get("id")?)
        }
        ConnectionType::Mysql(mut conn) => {
            let row = sqlx::query(sql).bind(email).fetch_one(&mut conn).await?;
            Ok(row.try_get("id")?)
        }
        ConnectionType::Postgres(mut conn) => {
            let row = sqlx::query(sql).bind(email).fetch_one(&mut conn).await?;
            Ok(row.try_get("id")?)
        }
    }
}

pub async fn delete_user(
    conn: ConnectionType,
    email: &str,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        DELETE FROM user WHERE email = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(email)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_rowid().try_into().unwrap())
        }
        ConnectionType::Mysql(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(email)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_id())
        }
        ConnectionType::Postgres(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(email)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.rows_affected())
        }
    }
}

// 测试
#[tokio::test]
async fn test_register_user() {
    use crate::lib::config::HttpServerConfig;
    let conn = crate::lib::config::init_db(&HttpServerConfig::default()).await;
    let users = get_all_user(conn).await.unwrap();
    println!("{:?}", users);
}
