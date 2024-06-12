use crate::lib::config::ConnectionType;

// 创建玩家账号绑定表
pub async fn create_player_table(conn: ConnectionType) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS java_player (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uid INTEGER NOT NULL,
            name TEXT NOT NULL UNIQUE,
            player_id TEXT NOT NULL
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

// 添加绑定玩家账号
pub async fn sql_add_player(
    conn: ConnectionType,
    uid: i64,
    name: &str,
    player_id: &str,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        INSERT INTO java_player (uid, name,player_id) VALUES (?, ?, ?);
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&uid)
                .bind(&name)
                .bind(&player_id)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_rowid().try_into().unwrap())
        }
        ConnectionType::Mysql(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&uid)
                .bind(&name)
                .bind(&player_id)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_id())
        }
        ConnectionType::Postgres(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&uid)
                .bind(&name)
                .bind(&player_id)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.rows_affected())
        }
    }
}

// 获取玩家账号
pub async fn sql_get_player(
    conn: ConnectionType,
    uid: i64,
) -> Result<Vec<(i64, String, String)>, sqlx::Error> {
    let sql = r#"
        SELECT * FROM java_player WHERE uid = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let player = sqlx::query_as::<_, (i64, String, String)>(sql)
                .bind(&uid)
                .fetch_all(&mut conn)
                .await?;
            Ok(player)
        }
        ConnectionType::Mysql(mut conn) => {
            let player = sqlx::query_as::<_, (i64, String, String)>(sql)
                .bind(&uid)
                .fetch_all(&mut conn)
                .await?;
            Ok(player)
        }
        ConnectionType::Postgres(mut conn) => {
            let player = sqlx::query_as::<_, (i64, String, String)>(sql)
                .bind(&uid)
                .fetch_all(&mut conn)
                .await?;
            Ok(player)
        }
    }
}
