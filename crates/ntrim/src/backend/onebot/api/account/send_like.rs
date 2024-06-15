use std::sync::Arc;
use bytes::{Bytes, BytesMut};
use serde_derive::Deserialize;
use serde_json::json;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct SendLikeParams {
    user_id: i64,
    times: i32
}

async fn handle_send_like(bot: &Arc<Bot>, params: SendLikeParams) -> actix_web::Result<impl serde::Serialize> {
    let _ = Bot::vote_user(bot, params.user_id, params.times, 2).await;
    Ok(json!({
        "user_id": params.user_id,
        "count": params.times
    }))
}

init_route!("/send_like", SendLikeParams, handle_send_like);