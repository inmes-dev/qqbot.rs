use std::sync::Arc;
use serde_derive::Deserialize;
use ntrim_core::await_response;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetQQProfileParams {
    user_id: Option<i64>
}

async fn handle_get_qq_profile(bot: &Arc<Bot>, params: GetQQProfileParams) -> actix_web::Result<impl serde::Serialize> {
    let user_id = params.user_id.unwrap_or(bot.unique_id);
    let profile = await_response!(tokio::time::Duration::from_secs(5), async {
        let rx = Bot::get_profile_detail(bot, user_id).await;
        if let Some(rx) = rx {
            rx.await.map_err(|e| anyhow::Error::from(e))
        } else {
            Err(anyhow::Error::msg("Unable to handle_get_qq_profile: tcp connection exception"))
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }).map_err(|e|
        OnebotError::InternalError(format!("Failed to handle_get_qq_profile: {}", e))
    )?;
    if profile.is_none() {
        return Err(Error::from(OnebotError::LogicError("User not found".to_string())));
    }
    Ok(profile.unwrap())
}

init_route!("/get_qq_profile", GetQQProfileParams, handle_get_qq_profile);