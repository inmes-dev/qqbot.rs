use std::sync::Arc;
use serde_derive::Deserialize;
use serde_json::json;
use ntrim_core::await_response;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct GetStrangerInfoParams {
    user_id: i64
}

async fn handle_get_stranger_info(bot: &Arc<Bot>, params: GetStrangerInfoParams) -> actix_web::Result<impl serde::Serialize> {
    let user_id = params.user_id;
    let summary_card = await_response!(tokio::time::Duration::from_secs(15), async {
        let rx = Bot::get_summary_card(bot, user_id).await;
        if let Some(rx) = rx {
            rx.await.map_err(|e| anyhow::Error::from(e))
        } else {
            Err(anyhow::Error::msg("Unable to handle_get_stranger_info: tcp connection exception"))
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }).map_err(|e|
        OnebotError::InternalError(format!("Failed to handle_get_stranger_info: {}", e))
    )?;
    if summary_card.is_none() {
        return Err(Error::from(OnebotError::LogicError("用户未找到或者未添加为好友".to_string())));
    }
    let summary_card = summary_card.unwrap();
    let nick_name = summary_card.nickname;
    let age = summary_card.age;
    let sex = match summary_card.sex {
        0 => "male",
        1 => "female",
        _ => "other"
    };
    let level = summary_card.level;
    let city = summary_card.city;
    let sign = summary_card.sign;
    let mobile = summary_card.mobile;
    let login_days = summary_card.login_days;
    Ok(json!({
        "user_id": user_id,
        "nickname": nick_name,
        "age": age,
        "sex": sex,
        "level": level,
        "city": city,
        "sign": sign,
        "mobile": mobile,
        "login_days": login_days
    }))
}

init_route!("/get_stranger_info", GetStrangerInfoParams, handle_get_stranger_info);