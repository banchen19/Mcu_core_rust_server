use crate::lib::config::ConnectionType;
use serde::{Deserialize, Serialize};
use sqlx::Row;

// 创建玩家账号绑定表
pub async fn create_player_table(conn: ConnectionType) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS java_player (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uid INTEGER NOT NULL,
            name TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL UNIQUE,
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
    password: &str,
    player_id: &str,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        INSERT INTO java_player (uid, name,password,player_id) VALUES (?, ?,?, ?);
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&uid)
                .bind(&name)
                .bind(&password)
                .bind(&player_id)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_rowid().try_into().unwrap())
        }
        ConnectionType::Mysql(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&uid)
                .bind(&name)
                .bind(&password)
                .bind(&player_id)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.last_insert_id())
        }
        ConnectionType::Postgres(mut conn) => {
            let quer_id = sqlx::query(sql)
                .bind(&uid)
                .bind(&name)
                .bind(&password)
                .bind(&player_id)
                .execute(&mut conn)
                .await?;
            Ok(quer_id.rows_affected())
        }
    }
}

// 获取玩家账号
pub async fn sql_get_player(conn: ConnectionType, password: &str) -> Result<String, sqlx::Error> {
    let sql: &str = r#"
        SELECT player_id FROM java_player WHERE password = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let player = sqlx::query(sql)
                .bind(&password)
                .fetch_one(&mut conn)
                .await?;
            let player_id: String = player.try_get(0)?;

            Ok(player_id)
        }
        ConnectionType::Mysql(mut conn) => {
            let player = sqlx::query(sql)
                .bind(&password)
                .fetch_one(&mut conn)
                .await?;
            let player_id: String = player.try_get(0)?;
            Ok(player_id)
        }
        ConnectionType::Postgres(mut conn) => {
            let player = sqlx::query(sql)
                .bind(&password)
                .fetch_one(&mut conn)
                .await?;
            let player_id: String = player.try_get(0)?;
            Ok(player_id)
        }
    }
}

// 获取玩家是否为正版
pub async fn sql_get_player_is_official(
    conn: ConnectionType,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let sql: &str = r#"
        SELECT player_id FROM java_player WHERE name = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let player = sqlx::query(sql).bind(&name).fetch_one(&mut conn).await?;
            let player_id: String = player.try_get(0)?;
            if player_id == "离线玩家" {
                return Ok(false);
            }
            Ok(true)
        }
        ConnectionType::Mysql(mut conn) => {
            let player = sqlx::query(sql).bind(&name).fetch_one(&mut conn).await?;
            let player_id: String = player.try_get(0)?;
            if player_id == "离线玩家" {
                return Ok(false);
            }
            Ok(true)
        }
        ConnectionType::Postgres(mut conn) => {
            let player = sqlx::query(sql).bind(&name).fetch_one(&mut conn).await?;
            let player_id: String = player.try_get(0)?;
            if player_id == "离线玩家" {
                return Ok(false);
            }
            Ok(true)
        }
    }
}

// 查询指定通uid下的player
#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    name: String,
    password: String,
}
pub async fn query_user(conn: ConnectionType, uid: i64) -> Result<Vec<Player>, sqlx::Error> {
    let sql: &str = r#"
        SELECT name,password FROM java_player WHERE uid = ?;
    "#;
    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let player = sqlx::query(sql).bind(&uid).fetch_all(&mut conn).await?;
            let mut player_list = Vec::new();
            for row in player {
                let player = Player {
                    name: row.try_get(0)?,
                    password: row.try_get(1)?,
                };
                player_list.push(player);
            }
            Ok(player_list)
        }
        ConnectionType::Mysql(mut conn) => {
            let player = sqlx::query(sql).bind(&uid).fetch_all(&mut conn).await?;
            let mut player_list = Vec::new();
            for row in player {
                let player = Player {
                    name: row.try_get(0)?,
                    password: row.try_get(1)?,
                };
                player_list.push(player);
            }
            Ok(player_list)
        }
        ConnectionType::Postgres(mut conn) => {
            let player = sqlx::query(sql).bind(&uid).fetch_all(&mut conn).await?;
            let mut player_list = Vec::new();
            for row in player {
                let player = Player {
                    name: row.try_get(0)?,
                    password: row.try_get(1)?,
                };
                player_list.push(player);
            }
            Ok(player_list)
        }
    }
}

// 修改玩家密码-修改快捷密码
pub async fn sql_update_player_password(
    conn: ConnectionType,
    name: &str,
    password: &str,
    uid: i64,
) -> Result<(), sqlx::Error> {
    let sql: &str = r#"
        UPDATE java_player SET password = ? WHERE name = ? and uid = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(&password)
                .bind(&name)
                .bind(&uid)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(&password)
                .bind(&name)
                .bind(&uid)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(&password)
                .bind(&name)
                .bind(&uid)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
    }
}

// 删除玩家账号
pub async fn sql_delete_player(
    conn: ConnectionType,
    name: &str,
    uid: i64,
) -> Result<(), sqlx::Error> {
    let sql: &str = r#"
        DELETE FROM java_player WHERE name = ? and uid = ?;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(&name)
                .bind(&uid)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(&name)
                .bind(&uid)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(&name)
                .bind(&uid)
                .execute(&mut conn)
                .await?;
            Ok(())
        }
    }
}


//admin- 获取所有玩家账号
pub async fn sql_get_all_player(conn: ConnectionType) -> Result<Vec<Player>, sqlx::Error> {
    let sql: &str = r#"
        SELECT name,password FROM java_player;
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let player = sqlx::query(sql).fetch_all(&mut conn).await?;
            let mut player_list = Vec::new();
            for row in player {
                let player = Player {
                    name: row.try_get(0)?,
                    password: row.try_get(1)?,
                };
                player_list.push(player);
            }
            Ok(player_list)
        }
        ConnectionType::Mysql(mut conn) => {
            let player = sqlx::query(sql).fetch_all(&mut conn).await?;
            let mut player_list = Vec::new();
            for row in player {
                let player = Player {
                    name: row.try_get(0)?,
                    password: row.try_get(1)?,
                };
                player_list.push(player);
            }
            Ok(player_list)
        }
        ConnectionType::Postgres(mut conn) => {
            let player = sqlx::query(sql).fetch_all(&mut conn).await?;
            let mut player_list = Vec::new();
            for row in player {
                let player = Player {
                    name: row.try_get(0)?,
                    password: row.try_get(1)?,
                };
                player_list.push(player);
            }
            Ok(player_list)
        }
    }
}


#[tokio::test]
async fn test_sql_player() {
    use crate::lib::config::get_conn;
    use crate::lib::config::HttpServerConfig;

    let config = HttpServerConfig::default();

    let conn: ConnectionType = get_conn(&config).await.unwrap();
    let uid = 1;
    let player_list = query_user(conn, uid).await.unwrap();
    println!("{:?}", player_list);
}
