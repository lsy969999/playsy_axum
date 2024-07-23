use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::sync::broadcast;
use uuid::Uuid;


#[derive(Clone)]
pub struct WsChatExtension {
    inner: Arc<Mutex<WsChatExtensionInner>>
}

impl WsChatExtension {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(
                    Mutex::new(
                            WsChatExtensionInner {
                                chat_room_lobby: ChatRoomLobby {
                                    rooms: HashMap::new(),
                                    rooms_tx: broadcast::Sender::<String>::new(100),
                                },
                            }
                        )
                    )
        }
    }

    pub fn run_room_checker(ws_state: WsChatExtension) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                ws_state.destroyroom();
            }
        });
    }

    pub fn destroyroom(&self) {
        let mut guard: std::sync::MutexGuard<WsChatExtensionInner> = self.inner.lock().unwrap();
        let rooms = &mut guard.chat_room_lobby.rooms;
        rooms.iter_mut().for_each(|(k,v)| {
            if v.chatters.len() == 0 {
                let now = OffsetDateTime::now_utc();
                if v.zero_user_time.is_none() {
                    return;
                }
                let zerotime = OffsetDateTime::from_unix_timestamp(v.zero_user_time.unwrap() as i64).unwrap();
                if now > (zerotime + Duration::from_secs(10)) {
                    v.destory_cnt -= 1;
                    if v.destory_cnt > 0 {
                        v.chats.push(ChatMsg {
                            room_id: k.to_string(),
                            who: "System".to_string(),
                            time: OffsetDateTime::now_utc().unix_timestamp() as usize,
                            msg: format!("방 폭파 시퀀스 가동중... 폭파 {}초전", v.destory_cnt)
                        });
                        let _ = v.chats_tx.send("chat".to_string());
                    } else {
                        v.chats.push(ChatMsg {
                            room_id: k.to_string(),
                            who: "System".to_string(),
                            time: OffsetDateTime::now_utc().unix_timestamp() as usize,
                            msg: format!("폭파")
                        });
                        let _ = v.chats_tx.send("chat".to_string());
                        let _ = v.chats_tx.send("destory".to_string());
                    }
                    
                }
            }
        });
        let keys_to_remove = rooms.iter()
            .filter(|(_k,v)|{v.destory_cnt == -1})
            .map(|(&k, _)| k)
            .collect::<Vec<_>>();
        keys_to_remove.iter()
            .for_each(|k|{
                rooms.remove(k);
            });
        if keys_to_remove.len() > 0 {
            
            let _ = guard.chat_room_lobby.rooms_tx.send("refresh".to_string());
        }
    }

    pub fn room_lobby_tx_clone(&self) -> broadcast::Sender<String> {
        let guard = self.inner.lock().unwrap();
        let tx = guard.chat_room_lobby.rooms_tx.clone();
        tx
    }

    pub fn create_room(&self, owner_sn: u32, owner: String, room_name: String ) {
        let uuid = Uuid::new_v4();
        let room = ChatRoom {
            owner: owner,
            owner_sn: owner_sn,
            chatters: Vec::new(),
            room_id: uuid,
            room_name: room_name,
            chats: Vec::new(),
            chats_tx: broadcast::Sender::<String>::new(100),
            create_at: OffsetDateTime::now_utc().unix_timestamp() as usize,
            destory_cnt: 10,
            zero_user_time: Some(OffsetDateTime::now_utc().unix_timestamp() as usize)
        };

        {
            self.inner.lock().unwrap().chat_room_lobby.rooms.insert(uuid, room);
        }
    }

    pub fn remove_room(&self, _room_id: Uuid) {
        todo!()
    }

    pub fn room_tx_clone(&self, room_id: &Uuid) -> broadcast::Sender<String> {
        let guard = self.inner.lock().unwrap();
        let room = guard.chat_room_lobby.rooms.get(&room_id).unwrap();
        let tx = room.chats_tx.clone();
        tx
    }

    pub fn get_all_room(&self) -> HashMap<Uuid, ChatRoom> {
        let guard = self.inner.lock().unwrap();
        
        let rooms = guard.chat_room_lobby.rooms.clone();
        // tracing::debug!("get_all_room {:?}", rooms);
        rooms
    }
    
    pub fn join_room(&self, room_id: &Uuid, user_sn: u32, user_name: String) {
        let mut guard = self.inner.lock().unwrap();
        let room = guard.chat_room_lobby.rooms.get_mut(&room_id);
        match room {
            Some(room) => {
                room.zero_user_time = None;
                room.destory_cnt = 10;
                room.chatters.push(Chatter { who: user_name, sn: user_sn });
                let _ = guard.chat_room_lobby.rooms_tx.send(format!("refresh"));
            }
            None => {
                let _ = guard.chat_room_lobby.rooms_tx.send(format!("refresh"));
            }
        }
    }

    pub fn get_chatter(&self, room_id: &Uuid, user_sn: u32) -> Option<Chatter> {
        let mut guard = self.inner.lock().unwrap();
        let room = guard.chat_room_lobby.rooms.get_mut(room_id);

        match room {
            Some(room) => {
                let a = &room.chatters;
                if let Some(pos) = a.iter().position(|x| x.sn == user_sn) {
                    Some(room.chatters.get(pos).unwrap().clone())
                } else {
                    None
                }
            }
            None => {
                None
            }
        }
    }

    pub fn leave_room(&self, room_id: &Uuid, user_sn: u32) {
        let mut guard = self.inner.lock().unwrap();
        let room = guard.chat_room_lobby.rooms.get_mut(&room_id);

        match room {
            Some(room) => {
                if let Some(pos) = room.chatters.iter().position(|x| x.sn == user_sn) {
                    room.chatters.remove(pos);
                }
                if room.chatters.len() == 0 {
                    room.zero_user_time = Some(OffsetDateTime::now_utc().unix_timestamp() as usize);
                }
                let _ = guard.chat_room_lobby.rooms_tx.send(format!("refresh"));
            }
            None => {
                let _ = guard.chat_room_lobby.rooms_tx.send(format!("refresh"));
            }
        }
    }

    pub fn room_owner_change() {

    }

    pub fn get_all_chat(&self, room_id: &Uuid) -> Vec<ChatMsg> {
        let guard = self.inner.lock().unwrap();
        let room = guard.chat_room_lobby.rooms.get(room_id);
        match room {
            Some(room) => {
                room.chats.clone()
            }
            None => {
                Vec::new()
            }
        }
    }

    pub fn get_last_chat(&self, room_id: &Uuid) -> Option<ChatMsg>{
        let guard = self.inner.lock().unwrap();
        match guard.chat_room_lobby.rooms.get(room_id) {
            Some(r) => {
                match r.chats.last() {
                    Some(c) => Some(c.clone()),
                    None => None,
                }
            }
            None => None
        }
        
    }

    pub fn chat(&self, room_id: &Uuid, who: String, msg: String) {
        let mut guard = self.inner.lock().unwrap();
        let room = guard.chat_room_lobby.rooms.get_mut(room_id);
        match room {
            Some(room) => {
                if room.chats.len() >= 500 {
                    room.chats.remove(0);
                }
                room.chats.push(ChatMsg {
                    room_id: room_id.to_string(),
                    who: who,
                    time: OffsetDateTime::now_utc().unix_timestamp() as usize,
                    msg: msg
                });
            }
            None => {
                tracing::warn!("none room chat push");
            }
        }
        
    }
}

struct WsChatExtensionInner {
    pub chat_room_lobby: ChatRoomLobby
}


#[derive(Debug, Clone)]
pub struct ChatRoomLobby {
    pub rooms: HashMap<Uuid, ChatRoom>,
    pub rooms_tx: broadcast::Sender<String>
}


#[derive(Debug, Clone)]
pub struct ChatRoom {
    pub owner_sn: u32,
    pub owner: String,
    pub chatters: Vec<Chatter>,
    pub room_name: String,
    pub room_id: Uuid,
    pub chats: Vec<ChatMsg>,
    pub chats_tx: broadcast::Sender<String>,
    pub create_at: usize,
    pub destory_cnt: i8,
    pub zero_user_time: Option<usize>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMsg {
    pub room_id: String,
    pub who: String,
    pub time: usize,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chatter{
    pub who: String,
    pub sn: u32,
}