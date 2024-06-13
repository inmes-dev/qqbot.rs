use std::sync::Arc;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use ntrim_core::await_response;
use ntrim_core::bot::Bot;
use ntrim_core::commands::troop::GroupMemberPermission;
use ntrim_core::db::{group_list, GroupMemberInfo, PG_POOL};
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetGroupMemberListParams {
    group_id: i64,
    refresh: Option<bool>
}

fn encode_group_member_list(group_member_list: Vec<GroupMemberInfo>) -> Vec<Value> {
    group_member_list.into_iter().map(|info| {
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
            "distance": 100,
            "honor": info.honor,
            "join_time": info.join_time,
            "last_active_time": info.last_speak_time,
            "last_sent_time": info.last_speak_time,
            "unique_name": info.special_title,
            "area": "",
            "level": info.level,
            "role": role,
            "unfriendly": false,
            "card_changeable": false
        })
    }).collect::<Vec<_>>()
}

async fn handle_get_group_member_list(bot: &Arc<Bot>, params: GetGroupMemberListParams) -> actix_web::Result<impl serde::Serialize> {
    #[cfg(feature = "sql")]
    if ntrim_core::db::is_initialized() && !params.refresh.unwrap_or(false) {
        let pool = PG_POOL.get().unwrap();
        let group_mem_list = GroupMemberInfo::query_by_group_id(pool, params.group_id).await
            .map_err(|e| OnebotError::InternalError(format!("Fetch group_member_list from sql failed: {}", e)))?;
        return Ok(encode_group_member_list(group_mem_list))
    }

    let troop_info = Bot::get_troop_info(bot, params.group_id).await
        .map_err(|e| OnebotError::InternalError(format!("Fetch group_info failed: {}", e)))?;
    let member_list = await_response!(tokio::time::Duration::from_secs(60), async {
        Bot::get_troop_member_list(bot, params.group_id, troop_info.owner_uin).await
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }).map_err( |e|
        OnebotError::InternalError(format!("Failed to get_troop_member_list: {}", e))
    )?;

    Ok(encode_group_member_list(member_list))
}

init_route!("/get_group_member_list", GetGroupMemberListParams, handle_get_group_member_list);