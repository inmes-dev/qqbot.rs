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
    let group_info_list = await_response!(tokio::time::Duration::from_secs(30), async {
        let rx = Bot::get_troop_info(bot, vec![params.group_id]).await;
        if let Some(rx) = rx {
            rx.await.map_err(|e| anyhow::Error::from(e))
        } else {
            Err(anyhow::Error::msg("Unable to handle_get_group_info: tcp connection exception"))
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }).map_err(|e|
        OnebotError::InternalError(format!("Failed to handle_get_group_info: {}", e))
    )?.ok_or(OnebotError::InternalError("GetTroopInfo result as null".to_string()))?;
    let group_info = group_info_list.first().ok_or(OnebotError::InternalError("Failed to get group information".to_string()))?;
    Ok(group_info.clone())
}

init_route!("/get_group_info", GetGroupInfoParams, handle_get_group_info);