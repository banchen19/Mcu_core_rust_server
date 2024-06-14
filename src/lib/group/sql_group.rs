use crate::lib::config::ConnectionType;

// 创建权限组表
pub async fn create_group_table(conn: ConnectionType) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS `groups` (
            `id` INTEGER PRIMARY KEY AUTOINCREMENT,
            `group_name` TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS `perm` (
            `group_name` TEXT NOT NULL,
            `perm` TEXT  TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS `user_perm` (
            `uid` INTEGER NOT NULL,
            `group_name` TEXT NOT NULL
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

// 添加权限组
pub async fn add_group(conn: ConnectionType, group_name: &str) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO `groups` (group_name) VALUES (?);
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql).bind(group_name).execute(&mut conn).await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql).bind(group_name).execute(&mut conn).await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql).bind(group_name).execute(&mut conn).await?;
            Ok(())
        }
    }
}

// 删除权限组
pub async fn del_group(conn: ConnectionType, group_name: &str) -> Result<(), sqlx::Error> {
    let sql = r#"
        DELETE FROM `groups` WHERE group_name = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql).bind(group_name).execute(&mut conn).await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql).bind(group_name).execute(&mut conn).await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql).bind(group_name).execute(&mut conn).await?;
            Ok(())
        }
    }
}

// 权限组添加权限
pub async fn add_perm(
    conn: ConnectionType,
    group_name: &str,
    perm: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO perm (group_name, perm) VALUES (?, ?);
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
    }
}

// 删除权限
pub async fn del_perm(conn: ConnectionType, perm: &str) -> Result<(), sqlx::Error> {
    let sql = r#"
        DELETE FROM perm WHERE perm = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql).bind(perm).execute(&mut conn).await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql).bind(perm).execute(&mut conn).await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql).bind(perm).execute(&mut conn).await?;
            Ok(())
        }
    }
}

// 为权限组添加权限
pub async fn add_perm_to_group(
    conn: ConnectionType,
    group_name: &str,
    perm: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO perm (group_name, perm) VALUES (?, ?);
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
    }
}

// 删除权限组的权限
pub async fn del_perm_from_group(
    conn: ConnectionType,
    group_name: &str,
    perm: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        DELETE FROM perm WHERE group_name = ? AND perm = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(group_name)
                .bind(perm)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
    }
}

// 查询所有权限组
pub async fn query_all_group(conn: ConnectionType) -> Result<Vec<String>, sqlx::Error> {
    let sql = r#"
        SELECT group_name FROM `groups`;
    "#;
    use sqlx::Row;
    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let rows = sqlx::query(sql).fetch_all(&mut conn).await?;
            let mut groups = Vec::new();
            for row in rows {
                groups.push(row.try_get("group_name")?);
            }
            Ok(groups)
        }
        ConnectionType::Mysql(mut conn) => {
            let rows = sqlx::query(sql).fetch_all(&mut conn).await?;
            let mut groups = Vec::new();
            for row in rows {
                groups.push(row.try_get("group_name")?);
            }
            Ok(groups)
        }
        ConnectionType::Postgres(mut conn) => {
            let rows = sqlx::query(sql).fetch_all(&mut conn).await?;
            let mut groups = Vec::new();
            for row in rows {
                groups.push(row.try_get("group_name")?);
            }
            Ok(groups)
        }
    }
}

// 查询权限组的权限
pub async fn query_group_perm(
    conn: ConnectionType,
    group_name: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let sql = r#"
        SELECT perm FROM perm WHERE group_name = ?;
    "#;
    use sqlx::Row;
    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let rows = sqlx::query(sql)
                .bind(group_name)
                .fetch_all(&mut conn)
                .await?;
            let mut perms = Vec::new();
            for row in rows {
                perms.push(row.try_get("perm")?);
            }
            Ok(perms)
        }
        ConnectionType::Mysql(mut conn) => {
            let rows = sqlx::query(sql)
                .bind(group_name)
                .fetch_all(&mut conn)
                .await?;
            let mut perms = Vec::new();
            for row in rows {
                perms.push(row.try_get("perm")?);
            }
            Ok(perms)
        }
        ConnectionType::Postgres(mut conn) => {
            let rows = sqlx::query(sql)
                .bind(group_name)
                .fetch_all(&mut conn)
                .await?;
            let mut perms = Vec::new();
            for row in rows {
                perms.push(row.try_get("perm")?);
            }
            Ok(perms)
        }
    }
}

// 给用户添加权限组
pub async fn add_group_to_user(
    conn: ConnectionType,
    uid: i64,
    group_name: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO user_perm (uid, group_name) VALUES (?, ?);
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(group_name)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(group_name)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(group_name)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
    }
}

// 删除用户的权限组
pub async fn del_group_from_user(
    conn: ConnectionType,
    uid: i64,
    group_name: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        DELETE FROM user_perm WHERE uid = ? AND group_name = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(group_name)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(group_name)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(group_name)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_create_group_table() {
    let config = crate::lib::config::read_yml("config.yml").unwrap();
    let conn = crate::lib::config::get_conn(&config).await.unwrap();
}
