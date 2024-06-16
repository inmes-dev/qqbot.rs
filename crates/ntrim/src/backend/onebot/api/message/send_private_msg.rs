use std::sync::Arc;
use bytes::{Bytes, BytesMut};
use dashmap::DashMap;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use ntrim_core::bot::Bot;
use ntrim_core::Contact;
use ntrim_tools::cqp::{parse_cq, parse_single_segment};
use ntrim_tools::cqp::parse_segments;
use crate::backend::UID_UIN_MAP;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct SendPrivateMessageParams {
    user_id: i64,
    message: Value,
    auto_escape: Option<bool>,
    recall_duration: Option<i64>
}

async fn handle_send_private_msg(bot: &Arc<Bot>, params: SendPrivateMessageParams) -> actix_web::Result<impl serde::Serialize> {
    let msg = match params.message {
        Value::Null => {
            return Err(Error::from(OnebotError::internal("Humorous message, give me a null? Null values are not supported.")))
        },
        Value::Bool(_) => {
            return Err(Error::from(OnebotError::internal("Boolean value received. Please provide an array or object pr string.")))
        },
        Value::Number(_) => {
            return Err(Error::from(OnebotError::internal("Boolean value received. Please provide an array or object or string.")))
        }
        Value::String(cq) => parse_cq(cq.as_bytes()),
        Value::Array(..) => parse_segments(params.message),
        Value::Object(..) => parse_single_segment(params.message).map(|scq| vec![scq])
    }.map_err(|e| OnebotError::InternalError(format!("Failed to parse message: {}", e)))?;
    let uid = if UID_UIN_MAP.get(&params.user_id).is_none() {
        let friend_list = Bot::get_friend_list(&bot, false).await
            .map_err(|e| OnebotError::InternalError(
                format!("Failed to get uid via friend_list: {}", e)
            ))?;
        let friend_info = friend_list.friends.iter()
            .filter(|friend_info| friend_info.uin == params.user_id)
            .collect::<Vec<_>>();
        let cache_mode = std::env::var("UID_CACHE_MODE").map_or("REVALIDATE".to_string(), |v| v);
        if cache_mode != "NONE" {
            UID_UIN_MAP.insert(params.user_id, friend_info[0].uid.clone());
        }
        friend_info[0].uid.clone()
    } else {
        UID_UIN_MAP.get(&params.user_id).unwrap().clone()
    };
    let result = Bot::send_msg(bot, Contact::Friend("".to_string(), 0, uid), msg).await
        .map_err(|e| OnebotError::InternalError(format!("Failed to send message: {}", e)))?;
    Ok(json!({
        "message_id": result
    }))
}

init_route!("/send_private_msg", SendPrivateMessageParams, handle_send_private_msg);