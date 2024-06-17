use std::sync::Arc;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use ntrim_core::bot::Bot;
use ntrim_core::Contact;
use ntrim_tools::cqp::{CQCode, parse_cq, parse_single_segment};
use ntrim_tools::cqp::parse_segments;
use crate::backend::UID_UIN_MAP;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct SendGroupMessageParams {
    group_id: i64,
    message: Value,
    auto_escape: Option<bool>,
    recall_duration: Option<i64>
}

async fn handle_send_group_msg(bot: &Arc<Bot>, params: SendGroupMessageParams) -> actix_web::Result<impl serde::Serialize> {
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
        Value::String(cq) => if params.auto_escape.unwrap_or(false) {
            Ok(vec![CQCode::Text(cq)])
        } else {
            parse_cq(cq.as_bytes())
        },
        Value::Array(..) => parse_segments(params.message),
        Value::Object(..) => parse_single_segment(params.message).map(|scq| vec![scq])
    }.map_err(|e| OnebotError::InternalError(format!("Failed to parse message: {}", e)))?;
    let result = Bot::send_msg(bot, Contact::Group("".to_string(), params.group_id), msg).await
        .map_err(|e| OnebotError::InternalError(format!("Failed to send message: {}", e)))?;
    Ok(json!({
        "message_id": result
    }))
}

init_route!("/send_group_msg", SendGroupMessageParams, handle_send_group_msg);