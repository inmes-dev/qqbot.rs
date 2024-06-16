use std::sync::Arc;
use serde_derive::Deserialize;
use serde_json::json;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetFriendListParams {
    refresh: Option<bool>
}

async fn handle_get_friend_list(bot: &Arc<Bot>, params: GetFriendListParams) -> actix_web::Result<impl serde::Serialize> {
    let friend_list = Bot::get_friend_list(bot, params.refresh.unwrap_or(false)).await
        .map_err(|e| OnebotError::InternalError(format!("Failed to get_friend_list: {}", e)))?;
    let friend_list = friend_list.friends.into_iter().map(|friend_info| {
        json!({
            "user_id": friend_info.uin,
            "user_name": friend_info.nick,
            "user_displayname": friend_info.remark,
            "user_remark": friend_info.remark,
            "age": 0,
            "gender": -1,
            "group_id": friend_info.group_id,
            "user_uid": friend_info.uid,
            "platform": "",
            "term_type": 0
        })
    }).collect::<Vec<_>>();
    Ok(friend_list)
}

init_route!("/get_friend_list", GetFriendListParams, handle_get_friend_list);