use std::sync::Arc;
use bytes::{BufMut, Bytes, BytesMut};
use serde_derive::Deserialize;
use ntrim_core::bot::Bot;
use crate::init_route;

#[derive(Deserialize, Debug)]
struct SetQQProfileParams {
    nickname: String,
    company: String,
    email: String,
    college: String,
    personal_note: String,
    age: Option<i32>
}

async fn handle_set_qq_profile(bot: &Arc<Bot>, params: SetQQProfileParams) -> actix_web::Result<impl serde::Serialize> {
    let user_id = bot.unique_id;
    let mut profile_card = Vec::new();
    profile_card.push((20002, Bytes::from(params.nickname.clone())));
    profile_card.push((20011, Bytes::from(params.email.clone())));
    profile_card.push((20019, Bytes::from(params.personal_note.clone())));
    profile_card.push((24008, Bytes::from(params.company.clone())));
    profile_card.push((20021, Bytes::from(params.college.clone())));
    if let Some(age) = params.age {
        let mut buf = BytesMut::new();
        buf.put_u16(age as u16);
        profile_card.push((20037, buf.freeze()));
    }

    Bot::set_profile_detail(bot, profile_card).await;
    Ok(serde_json::json!({
        "user_id": user_id,
        "nickname": params.nickname,
        "company": params.company,
        "email": params.email,
        "college": params.college,
        "personal_note": params.personal_note,
        "age": params.age,
    }))
}

init_route!("/set_qq_profile", SetQQProfileParams, handle_set_qq_profile);