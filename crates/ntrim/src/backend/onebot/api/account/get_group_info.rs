use std::sync::Arc;
use serde_derive::Deserialize;
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
    Ok(group_info)
}

init_route!("/get_group_info", GetGroupInfoParams, handle_get_group_info);