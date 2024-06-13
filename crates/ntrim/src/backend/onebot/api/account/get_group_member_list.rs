use std::sync::Arc;
use serde_derive::Deserialize;
use ntrim_core::await_response;
use ntrim_core::bot::Bot;
use ntrim_core::db::{group_list, GroupMemberInfo, PG_POOL};
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetGroupMemberListParams {
    group_id: i64,
    refresh: Option<bool>
}

async fn handle_get_group_member_list(bot: &Arc<Bot>, params: GetGroupMemberListParams) -> actix_web::Result<impl serde::Serialize> {
    #[cfg(feature = "sql")]
    if ntrim_core::db::is_initialized() && !params.refresh.unwrap_or(false) {
        let pool = PG_POOL.get().unwrap();
        let group_list = GroupMemberInfo::query_by_group_id(pool, params.group_id).await
            .map_err(|e| OnebotError::InternalError(format!("Fetch group_member_list from sql failed: {}", e)))?;
        return Ok(group_list)
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

    Ok(member_list)
}

init_route!("/get_group_member_list", GetGroupMemberListParams, handle_get_group_member_list);