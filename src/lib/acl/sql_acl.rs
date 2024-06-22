use std::collections::HashMap;

use serde::Serialize;

use crate::lib::{
    config::{get_conn, ConnectionType, HttpServerConfig},
    user::sql_user::get_user_id,
};

// 操作
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Operation {
    Add,
    Remove,
    Update,
    Check,
    None,
}

impl Operation {
    pub fn to_string(&self) -> String {
        match self {
            Operation::Add => String::from("Add"),
            Operation::Remove => String::from("Remove"),
            Operation::Update => String::from("Update"),
            Operation::Check => String::from("Check"),
            Operation::None => String::from("Null"),
        }
    }

    pub fn from_string(operation: &str) -> Self {
        match operation {
            "Add" => Operation::Add,
            "Remove" => Operation::Remove,
            "Update" => Operation::Update,
            "Check" => Operation::Check,
            _ => Operation::None,
        }
    }

    // 从Vec<Operation> 获取指定的操作
    pub fn get_operation(operations: Vec<Operation>, operation: &Operation) -> bool {
        operations.contains(operation)
    }

    pub fn default() -> String {
        "operation".to_string()
    }
}

// 资源
#[derive(Clone, Debug, Serialize)]
pub struct Resource {
    id: i64, // 资源id
    name: String,
}

impl Resource {
    pub fn default() -> String {
        "resource".to_string()
    }
}

pub async fn create_acl_table(conn: ConnectionType) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS acl (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        uid INTEGER NOT NULL,
        resource_id INTEGER NOT NULL,
        operation TEXT NOT NULL,
        UNIQUE(uid, resource_id, operation)
    );
     CREATE TABLE IF NOT EXISTS resource (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE
    );
    "#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(&sql).execute(&mut conn).await?;
            Ok(ConnectionType::Sqlite(conn))
        }

        ConnectionType::Mysql(mut conn) => {
            sqlx::query(&sql).execute(&mut conn).await?;
            Ok(ConnectionType::Mysql(conn))
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(&sql).execute(&mut conn).await?;
            Ok(ConnectionType::Postgres(conn))
        }
    }
}

// 添加资源
pub async fn add_resource(conn: ConnectionType, name: &str) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"INSERT INTO resource (name) VALUES ($1)"#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql).bind(name).execute(&mut conn).await?;
            Ok(ConnectionType::Sqlite(conn))
        }

        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql).bind(name).execute(&mut conn).await?;
            Ok(ConnectionType::Mysql(conn))
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql).bind(name).execute(&mut conn).await?;
            Ok(ConnectionType::Postgres(conn))
        }
    }
}

// 删除资源
pub async fn remove_resource(
    conn: ConnectionType,
    name: &str,
) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"DELETE FROM resource WHERE name = $1"#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql).bind(name).execute(&mut conn).await?;
            Ok(ConnectionType::Sqlite(conn))
        }

        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql).bind(name).execute(&mut conn).await?;
            Ok(ConnectionType::Mysql(conn))
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql).bind(name).execute(&mut conn).await?;
            Ok(ConnectionType::Postgres(conn))
        }
    }
}
// 添加用户对资源的操作
pub async fn add_acl(
    conn: ConnectionType,
    uid: i64,
    resource_id: i64,
    operation: &Operation,
) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"INSERT INTO acl (uid, resource_id, operation) VALUES ($1, $2, $3)"#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(resource_id)
                .bind(operation.to_string())
                .execute(&mut conn)
                .await.err();
            Ok(ConnectionType::Sqlite(conn))
        }

        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(resource_id)
                .bind(operation.to_string())
                .execute(&mut conn)
                .await.err();
            Ok(ConnectionType::Mysql(conn))
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(resource_id)
                .bind(operation.to_string())
                .execute(&mut conn)
                .await.err();
            Ok(ConnectionType::Postgres(conn))
        }
    }
}

// 移除用户对资源的操作
pub async fn remove_acl(
    conn: ConnectionType,
    uid: i64,
    resourceid: i64,
    operation: &str,
) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"DELETE FROM acl WHERE uid = $1 AND resourceid = $2 AND operation = $3"#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(resourceid)
                .bind(operation)
                .execute(&mut conn)
                .await?;
            Ok(ConnectionType::Sqlite(conn))
        }

        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(resourceid)
                .bind(operation)
                .execute(&mut conn)
                .await?;
            Ok(ConnectionType::Mysql(conn))
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(uid)
                .bind(resourceid)
                .bind(operation)
                .execute(&mut conn)
                .await?;
            Ok(ConnectionType::Postgres(conn))
        }
    }
}

// 移除用户对资源的操作-all
pub async fn remove_acl_by_resource_id(
    conn: ConnectionType,
    resource_id: i64,
) -> Result<ConnectionType, sqlx::Error> {
    let sql = r#"DELETE FROM acl WHERE resource_id = $1"#;

    match conn {
        ConnectionType::Sqlite(mut conn) => {
            sqlx::query(sql)
                .bind(resource_id)
                .execute(&mut conn)
                .await?;
            Ok(ConnectionType::Sqlite(conn))
        }

        ConnectionType::Mysql(mut conn) => {
            sqlx::query(sql)
                .bind(resource_id)
                .execute(&mut conn)
                .await?;
            Ok(ConnectionType::Mysql(conn))
        }
        ConnectionType::Postgres(mut conn) => {
            sqlx::query(sql)
                .bind(resource_id)
                .execute(&mut conn)
                .await?;
            Ok(ConnectionType::Postgres(conn))
        }
    }
}
// 获取用户对资源的操作
pub async fn get_acl(
    conn: ConnectionType,
    uid: u64,
    resource_id: i64,
) -> Result<Vec<Operation>, sqlx::Error> {
    let sql = r#"SELECT operation FROM acl WHERE uid = $1 AND resource_id = $2"#;
    use sqlx::Row;
    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let uid = uid as i64;
            let row_operation: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(sql)
                .bind(uid)
                .bind(resource_id)
                .fetch_all(&mut conn)
                .await?;

            let operations = row_operation
                .iter()
                .map(|row| {
                    let operation: String = row.try_get(0).unwrap();
                    Operation::from_string(&operation)
                })
                .collect::<Vec<Operation>>();

            Ok(operations)
        }

        ConnectionType::Mysql(mut conn) => {
            let row_operation = sqlx::query(sql)
                .bind(uid)
                .bind(resource_id)
                .fetch_all(&mut conn)
                .await?;
            let operations = row_operation
                .iter()
                .map(|row| {
                    let operation: String = row.try_get(0).unwrap();
                    Operation::from_string(&operation)
                })
                .collect::<Vec<Operation>>();

            Ok(operations)
        }
        ConnectionType::Postgres(mut conn) => {
            let uid = uid as i64;
            let row_operation = sqlx::query(sql)
                .bind(uid)
                .bind(resource_id)
                .fetch_all(&mut conn)
                .await?;
            let operations = row_operation
                .iter()
                .map(|row| {
                    let operation: String = row.try_get(0).unwrap();
                    Operation::from_string(&operation)
                })
                .collect::<Vec<Operation>>();

            Ok(operations)
        }
    }
}

// 获取资源id
pub async fn get_resource_id(
    conn: ConnectionType,
    name: &str,
) -> Result<(i64, ConnectionType), sqlx::Error> {
    let sql = r#"SELECT id FROM resource WHERE name = $1"#;
    use sqlx::Row;
    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let id = sqlx::query(sql).bind(name).fetch_one(&mut conn).await?;
            Ok((id.get(0), ConnectionType::Sqlite(conn)))
        }

        ConnectionType::Mysql(mut conn) => {
            let id = sqlx::query(sql).bind(name).fetch_one(&mut conn).await?;
            Ok((id.get(0), ConnectionType::Mysql(conn)))
        }
        ConnectionType::Postgres(mut conn) => {
            let id = sqlx::query(sql).bind(name).fetch_one(&mut conn).await?;
            Ok((id.get(0), ConnectionType::Postgres(conn)))
        }
    }
}

// 获取所有资源
pub async fn get_all_resource(conn: ConnectionType) -> Result<Vec<Resource>, sqlx::Error> {
    let sql = r#"SELECT id, name FROM resource"#;
    use sqlx::Row;
    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let rows = sqlx::query(sql).fetch_all(&mut conn).await?;
            let resources = rows
                .iter()
                .map(|row| {
                    let id: i64 = row.try_get(0).unwrap();
                    let name: String = row.try_get(1).unwrap();
                    Resource { id, name }
                })
                .collect::<Vec<Resource>>();
            Ok(resources)
        }

        ConnectionType::Mysql(mut conn) => {
            let rows = sqlx::query(sql).fetch_all(&mut conn).await?;
            let resources = rows
                .iter()
                .map(|row| {
                    let id: i64 = row.try_get(0).unwrap();
                    let name: String = row.try_get(1).unwrap();
                    Resource { id, name }
                })
                .collect::<Vec<Resource>>();
            Ok(resources)
        }
        ConnectionType::Postgres(mut conn) => {
            let rows = sqlx::query(sql).fetch_all(&mut conn).await?;
            let resources = rows
                .iter()
                .map(|row| {
                    let id: i64 = row.try_get(0).unwrap();
                    let name: String = row.try_get(1).unwrap();
                    Resource { id, name }
                })
                .collect::<Vec<Resource>>();
            Ok(resources)
        }
    }
}

// 初始化对资源操作权
pub async fn init_user_acl(
    config: &HttpServerConfig,
    uid: i64,
    resource: &str,
) -> Result<(), sqlx::Error> {
    let conn = get_conn(&config).await.unwrap();
    let resource_id = crate::lib::acl::sql_acl::get_resource_id(conn, resource)
        .await
        .unwrap();
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_acl(conn, uid, resource_id.0, &Operation::Add)
        .await
        .err();
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_acl(conn, uid, resource_id.0, &Operation::Remove)
        .await
        .err();
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_acl(conn, uid, resource_id.0, &Operation::Check)
        .await
        .err();
    Ok(())
}

// 查询用户对资源的操作
pub async fn query_user_acl(
    conn: ConnectionType,
    uid: u64,
) -> Result<HashMap<String, Vec<String>>, sqlx::Error> {
    let sql = r#"SELECT
    resource.name AS resource_name,
    acl.operation AS operation
FROM
    acl
JOIN
    resource ON resource.id = acl.resource_id
WHERE
    acl.uid = $1"#;
    use sqlx::Row;
    match conn {
        ConnectionType::Sqlite(mut conn) => {
            let uid = uid as i64;
            let row_operation = sqlx::query(sql).bind(uid).fetch_all(&mut conn).await?;
            let mut user_resources: HashMap<String, Vec<String>> = HashMap::new();
            for row in row_operation {
                let resource_name: String = row.try_get("resource_name")?;
                let operation: String = row.try_get("operation")?;
                if user_resources.contains_key(&resource_name) {
                    user_resources
                        .get_mut(&resource_name)
                        .unwrap()
                        .push(operation);
                } else {
                    user_resources.insert(resource_name, vec![operation]);
                }
            }
            Ok(user_resources)
        }
        ConnectionType::Mysql(mut conn) => {
            let uid = uid as i64;
            let row_operation = sqlx::query(sql).bind(uid).fetch_all(&mut conn).await?;
            let mut user_resources: HashMap<String, Vec<String>> = HashMap::new();
            for row in row_operation {
                let resource_name: String = row.try_get("resource_name")?;
                let operation: String = row.try_get("operation")?;
                if user_resources.contains_key(&resource_name) {
                    user_resources
                        .get_mut(&resource_name)
                        .unwrap()
                        .push(operation);
                } else {
                    user_resources.insert(resource_name, vec![operation]);
                }
            }
            Ok(user_resources)
        }
        ConnectionType::Postgres(mut conn) => {
            let uid = uid as i64;
            let row_operation = sqlx::query(sql).bind(uid).fetch_all(&mut conn).await?;
            let mut user_resources: HashMap<String, Vec<String>> = HashMap::new();
            for row in row_operation {
                let resource_name: String = row.try_get("resource_name")?;
                let operation: String = row.try_get("operation")?;
                if user_resources.contains_key(&resource_name) {
                    user_resources
                        .get_mut(&resource_name)
                        .unwrap()
                        .push(operation);
                } else {
                    user_resources.insert(resource_name, vec![operation]);
                }
            }
            Ok(user_resources)
        }
    }
}

#[tokio::test]
async fn test_acl() {
    use crate::HttpServerConfig;
    let config = HttpServerConfig::default();
    let conn = get_conn(&config).await.unwrap();
    let user_resource = query_user_acl(conn, 1).await.unwrap();
    println!("{:#?}", user_resource);
}
