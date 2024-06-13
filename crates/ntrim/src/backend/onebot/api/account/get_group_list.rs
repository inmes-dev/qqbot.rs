use std::sync::Arc;
use serde_derive::Deserialize;
use ntrim_core::await_response;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetGroupListParams {
    refresh: Option<bool>
}

async fn handle_get_group_list(bot: &Arc<Bot>, params: GetGroupListParams) -> actix_web::Result<impl serde::Serialize> {
    let group_info_list = await_response!(tokio::time::Duration::from_secs(30), async {
        match Bot::get_troop_list(bot, params.refresh.unwrap_or(false)).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e)
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }).map_err(|e|
        OnebotError::InternalError(format!("Failed to handle_get_group_list: {}", e))
    )?;
    Ok(group_info_list)
}

init_route!("/get_group_list", GetGroupListParams, handle_get_group_list);