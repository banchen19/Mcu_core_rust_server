use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::MigrateDatabase, Connection, MySqlConnection, PgConnection, Sqlite, SqliteConnection,
};

use super::{
    acl::sql_acl::{create_acl_table, Operation, Resource},
    java::player::sql_player::create_player_table,
    user::{
        sql_user::{change_password, create_user_table, get_user_id, register_user},
        web_user::RegisterUser,
    },
};

// 请求时消息
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage<'a> {
    pub code: u16,
    pub message: &'a str,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HttpServerConfig {
    pub name: String,
    pub v4port: u16,
    pub v6port: u16,
    pub sql_mode: SqlMode,
    pub sql_url: String,
    pub email_config: EmailConfig,
    pub register_user: RegisterUser,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct EmailConfig {
    pub mine_email: String,     // 发件人邮箱
    pub smtp_server: String,    // smtp服务器
    pub email_password: String, // 请使用授权码，而不是真实密码
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum SqlMode {
    sqlite,
    mysql,
    postgres,
}

impl Default for HttpServerConfig {
    fn default() -> Self {
        // 生成随机八个字符串
        let mut password = String::new();
        for _ in 0..8 {
            password.push((rand::random::<u8>() % 26 + 97) as char);
        }
        let file_path = "config.yml";
        let config = HttpServerConfig {
            name: "联合公社".to_string(), // 服务器名称
            v4port: 2024,
            v6port: 2024,
            sql_url: "sqlite://sqlite.db".to_string(),
            sql_mode: SqlMode::sqlite,
            email_config: EmailConfig::default(),
            register_user: RegisterUser {
                email: "admin".to_string(),
                password,
            },
        };
        match read_yml(&file_path) {
            Ok(config) => config,
            Err(_err) => {
                let _ = write_config_to_yml(&config, file_path);
                config
            }
        }
    }
}

// 写入到yml文件
pub fn write_config_to_yml(
    config: &HttpServerConfig,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_string = serde_yaml::to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(yaml_string.as_bytes())?;
    Ok(())
}

pub fn read_yml(file_path: &str) -> Result<HttpServerConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: HttpServerConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[derive(Debug)]
pub enum ConnectionType {
    Sqlite(SqliteConnection),
    Mysql(MySqlConnection),
    Postgres(PgConnection),
}

pub async fn get_conn(config: &HttpServerConfig) -> Result<ConnectionType, sqlx::Error> {
    match config.sql_mode {
        SqlMode::sqlite => {
            if config.sql_mode == SqlMode::sqlite {
                if !Sqlite::database_exists(&config.sql_url)
                    .await
                    .unwrap_or(false)
                {
                    match Sqlite::create_database(&config.sql_url).await {
                        Ok(_) => println!("Create db success"),
                        Err(error) => panic!("error: {}", error),
                    }
                }
                let conn: SqliteConnection = SqliteConnection::connect(&config.sql_url)
                    .await
                    .unwrap_or_else(|_| panic!("sqlite connect error"));
                Ok(ConnectionType::Sqlite(conn))
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        }
        SqlMode::mysql => {
            let conn = MySqlConnection::connect(&config.sql_url).await?;
            Ok(ConnectionType::Mysql(conn))
        }
        SqlMode::postgres => {
            let conn = PgConnection::connect(&config.sql_url).await?;
            Ok(ConnectionType::Postgres(conn))
        }
    }
}

pub(crate) async fn init_db(config: &HttpServerConfig) -> ConnectionType {
    let conn: ConnectionType = get_conn(config).await.unwrap();
    let conn = create_user_table(conn).await.unwrap();
    let conn = create_player_table(conn).await.unwrap();
    let conn = create_acl_table(conn).await.unwrap();
    init_base_data_acl(config).await;
    conn
}

// 初始化基本数据
pub async fn init_base_data_acl(config: &HttpServerConfig) {
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_resource(conn, "resource")
        .await
        .err();
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_resource(conn, "operation")
        .await
        .err();

    // 获取资源id
    let conn = get_conn(&config).await.unwrap();
    let resource_id = crate::lib::acl::sql_acl::get_resource_id(conn, &Resource::default())
        .await
        .unwrap();

    let conn = get_conn(&config).await.unwrap();
    let operation_id = crate::lib::acl::sql_acl::get_resource_id(conn, &Operation::default())
        .await
        .unwrap();

    let conn = get_conn(&config).await.unwrap();
    let uid = match register_user(conn, &config.register_user).await {
        Ok(uid) => uid.try_into().unwrap(),
        Err(_) => {
            // 修改密码
            let conn = get_conn(&config).await.unwrap();
            change_password(conn, &config.register_user).await.unwrap();
            let conn = get_conn(&config).await.unwrap();
            get_user_id(conn, &config.register_user.email)
                .await
                .unwrap()
                .try_into()
                .unwrap()
        }
    };
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_acl(conn, uid, resource_id, &Operation::Add)
        .await
        .err();
    let conn = get_conn(&config).await.unwrap();

    crate::lib::acl::sql_acl::add_acl(conn, uid, resource_id, &Operation::Remove)
        .await
        .err();
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_acl(conn, uid, resource_id, &Operation::Check)
        .await
        .err();

    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_acl(conn, uid, operation_id, &Operation::Add)
        .await
        .err();
    let conn = get_conn(&config).await.unwrap();

    crate::lib::acl::sql_acl::add_acl(conn, uid, operation_id, &Operation::Remove)
        .await
        .err();
    let conn = get_conn(&config).await.unwrap();
    crate::lib::acl::sql_acl::add_acl(conn, uid, operation_id, &Operation::Check)
        .await
        .err();
}

