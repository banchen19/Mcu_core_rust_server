use std::collections::HashMap;

use actix::Addr;
use actix_web::{web, HttpResponse};
use onlineplayer::{PlayerManager, PlayerUpdata, PlayersGet};

use crate::lib::config::{get_conn, HttpServerConfig, ResponseMessage};

pub mod chatserver;
pub mod onlineplayer;
pub mod sql_player;
pub mod web_player;

// 消息

// 玩家加入
pub async fn player_join(
    players: web::Data<Addr<PlayerManager>>,
    quer_user: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let server = quer_user.get("server").unwrap();
    let realname = quer_user.get("name").unwrap();
    if players
        .send(PlayerUpdata {
            r#type: "join".to_owned(),
            player: onlineplayer::OnlinePlayer {
                realname: realname.to_string(),
                server: server.to_string(),
            },
        })
        .await
        .unwrap()
    {
        return HttpResponse::Ok().json(ResponseMessage {
            code: 200,
            message: "加入成功",
        });
    } else {
        return HttpResponse::Ok().json(ResponseMessage {
            code: 300,
            message: "非法加入",
        });
    }
}

// 玩家离开
pub async fn player_leave(
    players: web::Data<Addr<PlayerManager>>,
    quer_user: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let server = quer_user.get("server").unwrap();
    let realname = quer_user.get("name").unwrap();
    let is_bool = players
        .send(PlayerUpdata {
            r#type: "leave".to_owned(),
            player: onlineplayer::OnlinePlayer {
                realname: realname.to_string(),
                server: server.to_string(),
            },
        })
        .await
        .unwrap();
    if is_bool {
        HttpResponse::Ok().json(ResponseMessage {
            code: 200,
            message: "离开成功",
        })
    } else {
        HttpResponse::Ok().json(ResponseMessage {
            code: 201,
            message: "玩家未在该服务器",
        })
    }
}

// 获取所有玩家
pub async fn get_players(players: web::Data<Addr<PlayerManager>>) -> HttpResponse {
    let players = players.send(PlayersGet {}).await.unwrap();
    HttpResponse::Ok().body(players)
}

// 获取所有服务端以及玩家
