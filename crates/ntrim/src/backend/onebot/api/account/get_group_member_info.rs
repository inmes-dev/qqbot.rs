use std::sync::Arc;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use ntrim_core::{*};
use ntrim_core::bot::Bot;
use ntrim_core::commands::troop::{GroupMemberInfo, GroupMemberPermission};
#[cfg(feature = "sql")]
use ntrim_core::db::{PG_POOL};
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetGroupMemberInfoParams {
    group_id: i64,
    user_id: i64,
    refresh: Option<bool>
}

fn encode_group_info(info: GroupMemberInfo) -> Value {
    let sex = match info.gender {
        0 => "male",
        1 => "female",
        _ => "other"
    };
    let role = match info.permission {
        GroupMemberPermission::Owner => "owner",
        GroupMemberPermission::Administrator => "admin",
        GroupMemberPermission::Member => "member"
    };
    json!({
        "user_id": info.uin,
        "group_id": info.group_code,
        "user_name": info.nickname,
        "sex": sex,
        "title": info.special_title,
        "title_expire_time": info.special_title_expire_time,
        "nickname": info.nickname,
        "user_displayname": info.card_name,
        "distance": info.distance,
        "honor": info.honor,
        "join_time": info.join_time,
        "last_active_time": info.last_speak_time,
        "last_sent_time": info.last_speak_time,
        "unique_name": info.special_title,
        "area": info.area,
        "level": info.level,
        "role": role,
        "unfriendly": false,
        "card_changeable": false
    })
}

async fn handle_get_group_member_info(bot: &Arc<Bot>, params: GetGroupMemberInfoParams) -> actix_web::Result<impl serde::Serialize> {
    let info = Bot::get_troop_member_card_info(bot, params.group_id, params.user_id, params.refresh).await
        .map_err(|e| OnebotError::InternalError(format!("Failed to get troop member card info: {}", e)))?;
    Ok(encode_group_info(info))
}

init_route!("/get_group_member_info", GetGroupMemberInfoParams, handle_get_group_member_info);