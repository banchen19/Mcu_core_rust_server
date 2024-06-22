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
    pub realname: String,
    pub server: String,
}

pub struct PlayerManager {
    // 服务器、玩家
    players: HashMap<String, Vec<String>>,
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
    pub fn add_player(&mut self, player: OnlinePlayer) -> bool {
        if self.players.contains_key(&player.server) {
            let mut server_players = self.players.get(&player.server).unwrap().clone();
            if !server_players.contains(&player.realname) {
                server_players.push(player.realname);
                if let Some(players) = self.players.get_mut(&player.server) {
                    *players = server_players.to_vec();
                    return true;
                }
            }
        } else {
            let mut server_players = Vec::new();
            server_players.push(player.realname);
            self.players.insert(player.server, server_players);
            return true;
        }
        false
    }
    /// 移除某个玩家
    pub fn remove_player(&mut self, player: OnlinePlayer) -> bool {
        let server_players = match self.players.get(&player.server) {
            Some(players) => players,
            None => return false,
        };
        if server_players.contains(&player.realname) {
            self.players
                .get_mut(&player.server)
                .unwrap()
                .retain(|name| name != &player.realname);
            true
        } else {
            false
        }
    }

    /// 获取所有玩家,带服务端名
    /// 返回预期：{"Bds": ["玩家名字2", "玩家名字"]}
    pub fn get_players(&self) -> HashMap<String, Vec<String>> {
        self.players.clone()
    }
    /// 移除所有玩家
    pub fn _remove_player_all(&mut self) {
        self.players.clear();
    }

    /// 删除指定服务端下的所有玩家
    fn remove_players_by_server(&mut self, server: &str) {
        self.players.retain(|k, _| k != server);
    }
}

// 接受消息
#[derive(Message)]
#[rtype(result = "bool")]
pub struct PlayerUpdata {
    pub(crate) r#type: String,
    pub player: OnlinePlayer,
}

impl Handler<PlayerUpdata> for PlayerManager {
    type Result = bool;

    fn handle(&mut self, player_join: PlayerUpdata, _: &mut Context<Self>) -> bool {
        if player_join.r#type == "join" {
            self.add_player(player_join.player)
        } else {
            self.remove_player(player_join.player)
        }
    }
}

#[derive(Message)]
#[rtype(result = "String")]
pub struct PlayersGet;
impl Handler<PlayersGet> for PlayerManager {
    type Result = String;

    fn handle(&mut self, _: PlayersGet, _: &mut Context<Self>) -> String {
        json!(self.get_players()).to_string()
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
