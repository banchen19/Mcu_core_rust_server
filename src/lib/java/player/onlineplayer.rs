use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// 广播数据
#[derive(Serialize, Deserialize, Clone)]
pub struct BroadcastData {
    /// 消息
    data: String,

    /// 请求者名字
    name: String,

    /// 服务器
    server: String,
}

pub struct OnlinePlayer {
    pub uid: i32,
    pub realname: String,
    pub server: String,
}

pub struct PlayerManager {
    // 玩家 服务器
    players: HashMap<i32, OnlinePlayer>,
}

impl Actor for PlayerManager {
    type Context = Context<Self>;
}

impl PlayerManager {
    pub fn new() -> PlayerManager {
        PlayerManager {
            players: HashMap::new(),
        }
    }
}

impl PlayerManager {
    /// 添加玩家
    pub fn add_player(&mut self, player: OnlinePlayer) {
        self.players.insert(player.uid, player);
    }
    /// 移除某个玩家
    pub fn remove_player(&mut self, player: OnlinePlayer) {
        self.players.remove(&player.uid);
    }
    /// 获取所有在线玩家名字的列表
    pub fn get_players(&self) -> Vec<String> {
        self.players
            .values()
            .map(|player| player.realname.clone())
            .collect()
    }

    /// 获取所有玩家,带服务端名
    /// 返回预期：{"Bds": ["玩家名字2", "玩家名字"]}
    pub fn get_players_with_server(&self) -> HashMap<String, Vec<String>> {
        let mut players_by_server: HashMap<String, Vec<String>> = HashMap::new();
        for player in self.players.values() {
            players_by_server
                .entry(player.server.clone())
                .or_insert_with(Vec::new)
                .push(player.realname.clone());
        }
        players_by_server
    }
    /// 移除所有玩家
    pub fn _remove_player_all(&mut self) {
        self.players.clear();
    }
    /// 删除指定服务端下的所有玩家
    fn remove_players_by_server(&mut self, server: &str) {
        self.players.retain(|_, player| player.server != server);
    }
}

// 接受消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerUpdata {
    pub(crate) r#type: String,
    pub player: OnlinePlayer,
}

impl Handler<PlayerUpdata> for PlayerManager {
    type Result = ();

    fn handle(&mut self, player_join: PlayerUpdata, _: &mut Context<Self>) {
        if player_join.r#type == "join" {
            self.add_player(player_join.player);
        } else {
            self.remove_player(player_join.player);
        }
    }
}

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct PlayersGet;
impl Handler<PlayersGet> for PlayerManager {
    type Result = Vec<String>;

    fn handle(&mut self, _: PlayersGet, _: &mut Context<Self>) -> Vec<String> {
        self.get_players()
    }
}

#[derive(Message)]
#[rtype(result = "String")]
pub struct PlayersGetWithServer;

impl Handler<PlayersGetWithServer> for PlayerManager {
    type Result = String;

    fn handle(&mut self, _: PlayersGetWithServer, _: &mut Context<Self>) -> String {
        json!(self.get_players_with_server()).to_string()
    }
}

// 清除指定服务端下的所有玩家
#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayersRemoveByServer {
    pub server: String,
}
impl Handler<PlayersRemoveByServer> for PlayerManager {
    type Result = ();

    fn handle(&mut self, players_remove_by_server: PlayersRemoveByServer, _: &mut Context<Self>) {
        self.remove_players_by_server(&players_remove_by_server.server);
    }
}
