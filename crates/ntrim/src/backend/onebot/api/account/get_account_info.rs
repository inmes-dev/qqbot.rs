use std::sync::Arc;
use serde_derive::Deserialize;
use tokio::sync::oneshot::Receiver;
use ntrim_core::await_response;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetAccountInfoParams {
    debug: Option<bool>
}

async fn handle_get_account_info(bot: &Arc<Bot>, params: GetAccountInfoParams) -> actix_web::Result<impl serde::Serialize> {
    let user_id = bot.unique_id;
    let profile = await_response!(tokio::time::Duration::from_secs(5), async {
        let rx = Bot::get_profile_detail(bot, user_id).await;
        if let Some(rx) = rx {
            rx.await.map_err(|e| anyhow::Error::from(e))
        } else {
            Err(anyhow::Error::msg("Unable to get_profile_detail: tcp connection exception"))
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }).map_err(|e|
        OnebotError::InternalError(format!("Failed to get_profile_detail: {}", e))
    )?;
    let nick_name = profile.map_or("unknown".to_string(), |p| p.nick_name);

    Ok(serde_json::json!({
        "user_id": user_id,
        "nickname": nick_name,
    }))
}

init_route!("/get_account_info", GetAccountInfoParams, handle_get_account_info);