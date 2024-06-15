use std::sync::Arc;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use ntrim_core::{await_response, db};
use ntrim_core::bot::Bot;
use ntrim_core::commands::troop::GroupMemberPermission;
use ntrim_core::db::{GroupMemberInfo, PG_POOL};
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
    #[cfg(feature = "sql")]
    if db::is_initialized() && !params.refresh.unwrap_or(false) {
        let pool = PG_POOL.get().unwrap();
        let group_member_info = GroupMemberInfo::query_member(pool, params.group_id, params.user_id).await
            .map_err(|e| OnebotError::InternalError(format!("Fetch group_member_info from sql failed: {}", e)))?;
        return Ok(encode_group_info(group_member_info))
    }
    let info = await_response!(tokio::time::Duration::from_secs(5), async {
        let rx = Bot::get_group_member_card_info(bot, params.group_id, params.user_id).await;
        if let Some(rx) = rx {
            rx.await.map_err(|e| anyhow::Error::from(e))
        } else {
            Err(anyhow::Error::msg("Unable to handle_get_group_member_info: tcp connection exception"))
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }).map_err(|e|
        OnebotError::InternalError(format!("Failed to handle_get_group_member_info: {}", e))
    )?.ok_or(OnebotError::internal("Get group member info is null"))?;
    Ok(encode_group_info(info))
}

init_route!("/get_group_member_info", GetGroupMemberInfoParams, handle_get_group_member_info);