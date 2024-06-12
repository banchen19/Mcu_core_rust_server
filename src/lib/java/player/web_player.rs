use std::collections::HashMap;

use actix_web::{http::header::AUTHORIZATION, web, HttpRequest, HttpResponse};
use reqwest::Client;

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
                                message: "已被绑定过",
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
                                message: "已被绑定过",
                            });
                        }
                    }
                }
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(ResponseMessage {
                code: 401,
                message: "token验证失败",
            });
        }
    }
}

// 验证账号是否绑定
pub async fn verify_player(config: web::Data<HttpServerConfig>, req: HttpRequest) -> HttpResponse {
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    match gettoken_to_user_no_time(token) {
        Ok(user) => {
            let uid = user.claims.uid;
            let conn = get_conn(&config).await.unwrap();
            let player = sql_player::sql_get_player(conn, uid.try_into().unwrap())
                .await
                .unwrap();
            if player.is_empty() {
                return HttpResponse::Ok().json(ResponseMessage {
                    code: 200,
                    message: "未绑定",
                });
            } else {
                return HttpResponse::Ok().json(ResponseMessage {
                    code: 200,
                    message: "已绑定",
                });
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(ResponseMessage {
                code: 401,
                message: "token验证失败",
            });
        }
    }
}
