use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use lib::{
    acl::web_acl,
    config::{get_conn, HttpServerConfig},
    java::player::{
        self, chatserver::{chatserver, session}, onlineplayer::PlayerManager, web_player
    },
    user::{
        email_code::{EmaiCodeManager, EmailManager},
        web_user,
    },
};
use log::info;

mod lib;

pub fn init_log() {
    use chrono::Local;
    use std::io::Write;
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    let mut builder = env_logger::Builder::from_env(env);
    builder
        .format(|buf, record| {
            let level = { buf.default_level_style(record.level()) };
            writeln!(
                buf,
                "{} {} {} [{}::{}] {:?}",
                format_args!("{:<5}", level),
                Local::now().format("%Y-%m-%d %T%.3f"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                std::thread::current().name().unwrap_or("<unnamed>"),
                record.args()
            )
        })
        .init();
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    init_log();

    // 初始化配置文件
    let config = HttpServerConfig::default();
    info!(" http server config: {:#?}", config);

    // 聊天服务器
    let server = chatserver::ChatServer::new().start();
    // 玩家管理
    let players = PlayerManager::new().start();

    // 邮箱验证码管理
    let email_code_manager = EmaiCodeManager::new().start();

    // 邮箱管理发送
    let emailmanager = EmailManager::new(
        config.name.clone(),
        config.email_config.smtp_server.clone(),
        config.email_config.mine_email.clone(),
        config.email_config.email_password.clone(),
    )
    .start();

    let v4port = config.v4port;
    let v6port = config.v6port;

    // 初始化数据库
    lib::config::init_db(&config).await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(players.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(email_code_manager.clone()))
            .app_data(web::Data::new(emailmanager.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(
                web::scope("/api").configure(|cfg: &mut web::ServiceConfig| {
                    cfg.service(
                        web::scope("/user")
                            // 获取验证码
                            .route("/get_code", web::get().to(web_user::get_code))
                            // token验证
                            .route("/token_verify", web::get().to(web_user::token_verify))
                            .route("/register", web::post().to(web_user::register))
                            .route("/login", web::post().to(web_user::login))
                            // 忘记密码
                            .route(
                                "/forget_password",
                                web::post().to(web_user::forget_password),
                            )
                            // *******************admin********************
                            // 获取所有用户
                            .route("/get_all", web::get().to(web_user::get_all))
                            // 修改用户密码
                            .route(
                                "/update_password",
                                web::post().to(web_user::change_password),
                            )
                            // 删除用户
                            .route("/delete", web::post().to(web_user::delete_user)),
                    );
                    cfg.service(
                        web::scope("/java").service(
                            web::scope("/player")
                                .route("/bind", web::post().to(web_player::add_bind_player))
                                // 验证账绑定了指定的玩家
                                .route("/login", web::get().to(web_player::login))
                                // 检查玩家是否为正版玩家
                                .route("/check_player", web::get().to(web_player::check_player))
                                // 查询拥有的玩家
                                .route("/query_player", web::get().to(web_player::query_player))
                                // 修改玩家密码
                                .route(
                                    "/update_password",
                                    web::post().to(web_player::update_player),
                                )
                                // 删除玩家
                                .route("/delete_player", web::post().to(web_player::delete_player))
                                // 在线玩家-加入
                                .route("/player_join", web::post().to(player::player_join))
                                // 在线玩家-查询在线玩家
                                .route("/players", web::get().to(player::get_players))
                                // 在线玩家-离开
                                .route("/player_leave", web::post().to(player::player_leave),
                                ),
                        ),
                    );
                    cfg.service(web::resource("/ws").route(web::get().to(session::ws_route)));

                    cfg.service(
                        web::scope("/acl")
                            .service(
                                web::scope("/resource")
                                    .route("/get_all", web::get().to(web_acl::acl_get_all_resource))
                                    // 添加资源
                                    .route("/add", web::post().to(web_acl::acl_add_resource))
                                    // 删除资源
                                    .route("/delete", web::post().to(web_acl::acl_delete_resource)),
                            )
                            .service(
                                web::scope("/operation")
                                    // 添加用户对资源的操作
                                    .route("/add", web::post().to(web_acl::acl_add_user_operation))
                                    // 删除用户对资源的操作
                                    .route(
                                        "/delete",
                                        web::post().to(web_acl::acl_remove_user_operation),
                                    ),
                            )
                            // 查询指定用户对资源的操作
                            .route("/query", web::get().to(web_acl::acl_get_user_operation)),
                    );
                }),
            )
    })
    .bind(("0.0.0.0", v4port))? //ipv4
    // .bind(("[::]", v6port))? //ipv6
    .run()
    .await
}
