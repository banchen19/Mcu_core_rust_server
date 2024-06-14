use std::collections::HashMap;

use actix_web::{http::header::AUTHORIZATION, web, HttpRequest, HttpResponse};
use log::info;
use reqwest::Client;
use serde_json::json;

use crate::lib::{
    config::{get_conn, HttpServerConfig, ResponseMessage},
    key::gettoken_to_user_no_time,
};

use super::sql_player;

// 添加绑定玩家账号
pub async fn add_bind_player(
    config: web::Data<HttpServerConfig>,
    quer_player: web::Query<HashMap<String, String>>,
    req: HttpRequest,
) -> HttpResponse {
    let player_name = quer_player.get("player_name").unwrap();
    let player_password = quer_player.get("password").unwrap();
    if player_name.len() <= 1 {
        return HttpResponse::Ok().json(ResponseMessage {
            code: 200,
            message: "玩家名字过短",
        });
    }
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    match gettoken_to_user_no_time(token) {
        Ok(user) => {
            let uid = user.claims.uid;
            // 业务逻辑
            let url = format!(
                "https://api.mojang.com/users/profiles/minecraft/{}",
                player_name
            );
            let client = Client::new();
            let response_result = client.get(url).send().await.unwrap();
            let response = response_result.text().await.unwrap();

            #[derive(serde::Deserialize)]
            struct Player {
                id: String,
                name: String,
            }

            match serde_json::from_str::<Player>(&response) {
                Ok(player) => {
                    let conn = get_conn(&config).await.unwrap();
                    match sql_player::sql_add_player(
                        conn,
                        uid.try_into().unwrap(),
                        &player.name,
                        player_password,
                        &player.id,
                    )
                    .await
                    {
                        Ok(_) => {
                            return HttpResponse::Ok().json(ResponseMessage {
                                code: 200,
                                message: "正版绑定成功",
                            });
                        }
                        Err(_) => {
                            return HttpResponse::Ok().json(ResponseMessage {
                                code: 200,
                                message: "正版账号->绑定失败,请检查密码或token是否合规,密码必须是唯一且只有自己知道",
                            });
                        }
                    }
                }
                Err(_) => {
                    let conn = get_conn(&config).await.unwrap();

                    match sql_player::sql_add_player(
                        conn,
                        uid.try_into().unwrap(),
                        player_name,
                        player_password,
                        "离线玩家",
                    )
                    .await
                    {
                        Ok(_) => {
                            return HttpResponse::Ok().json(ResponseMessage {
                                code: 200,
                                message: "离线绑定成功",
                            });
                        }
                        Err(_) => {
                            return HttpResponse::Ok().json(ResponseMessage {
                                code: 200,
                                message: "离线账号->绑定失败,请检查密码或token是否合规,密码必须是唯一且只有自己知道",
                            });
                        }
                    }
                }
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(ResponseMessage {
                code: 401,
                message: "token已过期",
            });
        }
    }
}

// 便捷密码登录
pub async fn login(
    config: web::Data<HttpServerConfig>,
    quer_user: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let password = quer_user.get("password").unwrap();
    let conn = get_conn(&config).await.unwrap();
    let player = sql_player::sql_get_player(conn, password).await;
    match player {
        Ok(_) => {
            return HttpResponse::Ok().json(ResponseMessage {
                code: 200,
                message: "登录成功",
            });
        }
        Err(_err) => {
            return HttpResponse::Created().json(ResponseMessage {
                code: 201,
                message: "登录失败",
            })
        }
    }
}

// 检查玩家是否为绑定玩家
pub async fn check_player(
    config: web::Data<HttpServerConfig>,
    quer_user: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let player_name = quer_user.get("player_name").unwrap();
    let conn = get_conn(&config).await.unwrap();
    let player = sql_player::sql_get_player_is_official(conn, player_name)
        .await
        .unwrap();

    if player {
        return HttpResponse::Ok().json(ResponseMessage {
            code: 200,
            message: "正版玩家",
        });
    } else {
        return HttpResponse::Created().json(ResponseMessage {
            code: 201,
            message: "离线玩家",
        });
    }
}

// 查询uid拥有的java账户
pub async fn query_player(config: web::Data<HttpServerConfig>, req: HttpRequest) -> HttpResponse {
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    // 验证token
    match gettoken_to_user_no_time(token) {
        Ok(user) => {
            let uid = user.claims.uid;
            let conn = get_conn(&config).await.unwrap();
            match sql_player::query_user(conn, uid.try_into().unwrap()).await {
                Ok(player_list) => {
                    return HttpResponse::Ok().json(player_list);
                }
                Err(_) => {
                    return HttpResponse::Ok().json(ResponseMessage {
                        code: 404,
                        message: "查询失败",
                    });
                }
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(ResponseMessage {
                code: 401,
                message: "token已过期",
            });
        }
    }
}

// 修改玩家快捷密码
pub async fn update_player(
    config: web::Data<HttpServerConfig>,
    quer_player: web::Query<HashMap<String, String>>,
    req: HttpRequest,
) -> HttpResponse {
    let player_name = quer_player.get("player_name").unwrap();
    let player_password = quer_player.get("password").unwrap();
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    match gettoken_to_user_no_time(token) {
        Ok(user) => {
            let uid = user.claims.uid;
            let conn = get_conn(&config).await.unwrap();
            match sql_player::sql_update_player_password(
                conn,
                player_name,
                player_password,
                uid.try_into().unwrap(),
            )
            .await
            {
                Ok(_) => {
                    return HttpResponse::Ok().json(ResponseMessage {
                        code: 200,
                        message: "修改成功",
                    });
                }
                Err(_) => {
                    return HttpResponse::Ok().json(ResponseMessage {
                        code: 200,
                        message: "修改失败",
                    });
                }
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(ResponseMessage {
                code: 401,
                message: "token已过期",
            });
        }
    }
}

// 删除绑定玩家
pub async fn delete_player(
    config: web::Data<HttpServerConfig>,
    quer_player: web::Query<HashMap<String, String>>,
    req: HttpRequest,
) -> HttpResponse {
    let player_name = quer_player.get("player_name").unwrap();
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    match gettoken_to_user_no_time(token) {
        Ok(user) => {
            let uid = user.claims.uid;
            let conn = get_conn(&config).await.unwrap();
            match sql_player::sql_delete_player(conn, player_name, uid.try_into().unwrap()).await {
                Ok(_) => {
                    return HttpResponse::Ok().json(ResponseMessage {
                        code: 200,
                        message: "删除成功",
                    });
                }
                Err(_) => {
                    return HttpResponse::Ok().json(ResponseMessage {
                        code: 200,
                        message: "删除失败",
                    });
                }
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(ResponseMessage {
                code: 401,
                message: "token已过期",
            });
        }
    }
}
