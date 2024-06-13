use std::sync::Arc;
use serde_derive::Deserialize;
use serde_json::json;
use ntrim_core::await_response;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetGroupInfoParams {
    group_id: i64
}

async fn handle_get_group_info(bot: &Arc<Bot>, params: GetGroupInfoParams) -> actix_web::Result<impl serde::Serialize> {
    let group_info = Bot::get_troop_info(bot, params.group_id).await
        .map_err(|e| OnebotError::InternalError(format!("{}", e)))?;

    Ok(json!({
        "group_id": group_info.code,
        "group_name": group_info.name,
        "group_remark": "",
        "group_uin": group_info.uin,
        "group_memo": group_info.memo,
        "owner": group_info.owner_uin,
        "group_create_time": group_info.group_create_time,
        "class_text": "",
        "is_frozen": false,
        "max_member": group_info.max_member_count,
        "member_num": group_info.member_count,
        "member_count": group_info.member_count,
        "max_member_count": group_info.max_member_count
    }))
}

init_route!("/get_group_info", GetGroupInfoParams, handle_get_group_info);