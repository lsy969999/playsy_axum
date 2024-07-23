use std::{str::FromStr, sync::Arc};
use axum::{extract::{ws::{Message, WebSocket}, Path, WebSocketUpgrade}, response::IntoResponse, Extension};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::{ configs::app_extensions::WsChatExtension, extractors::ext_user_info::ExtUserInfo, responses::html_template::HtmlTemplate, templates::chat::ChatTempalte};


pub async  fn chat_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse {
    HtmlTemplate(
        ChatTempalte {
            user_info
        }
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsData {
    r#type: String,
    name: String,
    body: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsCmdBodyCreateRoom {
    owner_sn: u32,
    owner: String,
    room_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsCmdChatRoomJoin {
    who: String,
    who_sn: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsCmdChatRoomLeave {
    who: String,
    who_sn: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsCmdChatRoomDestory {
    room_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsChatMsg {
    who: String,
    msg: String,
}

pub async fn ws_room_lobby_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<WsChatExtension>
) -> impl IntoResponse {
    ws.on_upgrade(|socket| ws_room_lobby(socket, state))
}

async fn ws_room_lobby(socket: WebSocket, state: WsChatExtension) {
    let (mut sender,
        mut receiver) = socket.split();
        

    fn refresh(state: WsChatExtension) -> String {
        let rooms = state.get_all_room();
        let v = rooms.iter().map(|(_key, val )| {
            serde_json::json!({
                "owner_sn": val.owner_sn,
                "chatters": val.chatters,
                "room_id": val.room_id.to_string(),
                "room_name": val.room_name,
                "num": val.chatters.len()
            })
        }).collect::<Vec<_>>();
        let rooms = serde_json::json!({
            "rooms": v
        });
        let ws_data = WsData {
            r#type: format!("cmd"),
            name: format!("create_room"),
            body: rooms
        };

        let sender_str = serde_json::to_string(&ws_data).unwrap();
        sender_str
    }

    let sender_str = refresh(state.clone());
    let _ = &sender.send(Message::Text(sender_str)).await;

    let cloned_state = state.clone();
    let mut rx = cloned_state.room_lobby_tx_clone().subscribe();
    let mut sender_task = tokio::spawn(async move {
        while let Ok(text) = rx.recv().await {
            if text == "refresh" {
                let sender_str = refresh(cloned_state.clone());
                let _ = &sender.send(Message::Text(sender_str)).await;
            }
        }
    });

    let tx = state.room_lobby_tx_clone();
    let mut receiver_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    let data = serde_json::from_str::<WsData>(&text).unwrap();
                    if data.r#type == "cmd" {
                        if data.name == "create_room" {
                            let create_room_cmd = serde_json::from_value::<WsCmdBodyCreateRoom>(data.body).unwrap();
                            state.create_room(create_room_cmd.owner_sn, create_room_cmd.owner, create_room_cmd.room_name);
                            let _ = tx.send(format!("refresh"));
                        }
                    }
                }
                msg => {
                    tracing::debug!("receiver_task not Text, {:?}", msg);
                }
            }
        }
    });

    tokio::select! {
        _ = &mut sender_task => receiver_task.abort(), 
        _ = &mut receiver_task => sender_task.abort(),
    }


}

pub async fn ws_room_handler(
    Path((roomid, usersn)): Path<(String, u32)>,
    ws: WebSocketUpgrade,
    Extension(state): Extension<WsChatExtension>
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_room(socket, state, roomid, usersn))
}

async fn ws_room(socket: WebSocket, state: WsChatExtension, roomid: String, usersn: u32) {
    tracing::debug!("usersn {}", usersn);
    let roomid = Uuid::from_str(&roomid).unwrap();
    let (sender,
        mut receiver) = socket.split();

    let arc_sender = Arc::new(tokio::sync::Mutex::new(sender));


    fn get_all_chat(state: WsChatExtension, roomid: &Uuid) -> String {
        let all_chat = state.get_all_chat(roomid);
        let chat  = serde_json::json!({
            "chats": all_chat
        });
        let ws_data = WsData {
            r#type: "cmd".to_owned(),
            name: "all_chat".to_owned(),
            body: chat
        };
        let sender_str = serde_json::to_string(&ws_data).unwrap();
        sender_str
    }

    let cloned_arc_sender = arc_sender.clone();
    let sender_str = get_all_chat(state.clone(), &roomid);
    let _ = cloned_arc_sender.lock().await.send(Message::Text(sender_str)).await;

    let (mpsc_tx, mut mpsc_rx) = mpsc::channel::<String>(100);
    let cloned_arc_sender = arc_sender.clone();
    let cloned_state = state.clone();
    let mut individual_sender_task = tokio::spawn(async move {
        while let Some(text) = mpsc_rx.recv().await {
            if text == "allchat" {
                let sender_str = get_all_chat(cloned_state.clone(), &roomid);
                let _ = cloned_arc_sender.lock().await.send(Message::Text(sender_str)).await;
            }
        }
    });

    let cloned_arc_sender = arc_sender.clone();
    let mut rx = state.room_tx_clone(&roomid).subscribe();
    let cloned_state = state.clone();
    let mut sender_task = tokio::spawn(async move {
        while let Ok(text) = rx.recv().await {
            if text == "chat" {
                let chat = cloned_state.get_last_chat(&roomid);
                tracing::debug!("chat: {:?}",  chat);
                if chat.is_some() {
                    let chat = chat.unwrap();
                    let chat  = serde_json::json!({
                        "chat": chat
                    });
                    let ws_data = WsData {
                        r#type: "cmd".to_owned(),
                        name: "last_chat".to_owned(),
                        body: chat
                    };
                    let sender_str = serde_json::to_string(&ws_data).unwrap();
                    {
                        let mut guard = cloned_arc_sender.lock().await;
                        if guard.send(Message::Text(sender_str)).await.is_err() {
                            break;
                        }
                    }
                }
            } else if text == "destory" {
                let destory_cmd = WsCmdChatRoomDestory {
                    room_id: roomid.clone().to_string()
                };
                let ws_data = WsData {
                    r#type: "cmd".to_owned(),
                    name: "room_destory".to_owned(),
                    body: serde_json::to_value(destory_cmd).unwrap()
                };
                let sender_str = serde_json::to_string(&ws_data).unwrap();
                {
                    let mut guard = cloned_arc_sender.lock().await;
                    if guard.send(Message::Text(sender_str)).await.is_err() {
                        break;
                    }
                }
                break;
            }
        }
    });

    let cloned_state = state.clone();
    let tx = cloned_state.room_tx_clone(&roomid);
    let mut receiver_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    let data = serde_json::from_str::<WsData>(&text).unwrap();
                    if data.r#type == "cmd" {
                        if data.name == "chat_join" {
                            let room_join_cmd = serde_json::from_value::<WsCmdChatRoomJoin>(data.body).unwrap();
                            cloned_state.join_room(&roomid, room_join_cmd.who_sn, room_join_cmd.who.clone());
                            cloned_state.chat(&roomid, format!("System"), format!("{} 님이 참가 했습니다.", room_join_cmd.who));
                            tracing::debug!("!!!! {:?}", room_join_cmd);
                            let _ = tx.send("chat".to_string());
                        } else if data.name == "chat_leave" {
                            let room_leave_cmd = serde_json::from_value::<WsCmdChatRoomLeave>(data.body).unwrap();
                            cloned_state.leave_room(&roomid, room_leave_cmd.who_sn);
                            cloned_state.chat(&roomid, format!("System"), format!("{} 님이 나갔습니다.", room_leave_cmd.who));
                            let _ = tx.send("chat".to_string());
                        } else if data.name == "get_all_chat" {
                            let _ = mpsc_tx.send("allchat".to_string()).await;
                        }
                    } else if data.r#type == "chat" {
                        if data.name == "msg" {
                            let chat = serde_json::from_value::<WsChatMsg>(data.body).unwrap();
                            cloned_state.chat(&roomid, format!("{}", chat.who), format!("{}", chat.msg));
                            
                            let _ = tx.send("chat".to_string());
                        }
                    }
                }
                msg => {
                    tracing::debug!("msg: {:?}", msg);
                }
            }
        }
    });

    tokio::select! {
        _ = &mut sender_task => {
            receiver_task.abort();
            individual_sender_task.abort();
        },
        _ = &mut receiver_task => {
            sender_task.abort();
            individual_sender_task.abort();
        },
        _ = &mut individual_sender_task => {
            sender_task.abort();
            receiver_task.abort();
        },
    };
    let cloned_state = state.clone();
    let chatter = cloned_state.get_chatter(&roomid, usersn).unwrap();
    cloned_state.leave_room(&roomid, usersn);
    cloned_state.chat(&roomid, format!("System"), format!("{} 님이 나갔습니다.", chatter.who));
    let tx = cloned_state.room_tx_clone(&roomid);
    let _ = tx.send("chat".to_string());
}