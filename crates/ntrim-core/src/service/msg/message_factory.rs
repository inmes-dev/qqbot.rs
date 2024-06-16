use std::sync::Arc;
use anyhow::{anyhow, Error};
use log::warn;
use prost::Message as _;
use ntrim_tools::cqp::CQCode;
use crate::bot::Bot;
use crate::Contact;
use crate::pb::msg::{ * };
use crate::pb::msg::text::TextReversed;

pub(crate) async fn convert_cq_to_msg(bot: &Arc<Bot>, contact: &Contact, cqs: Vec<CQCode>) -> RichText {
    let mut elems = vec![
        Elem {
            aio_elem: Some(elem::AioElem::GeneralFlags(
                GeneralFlags {
                    bubble_diy_text_id: Some(0),
                    bubble_sub_id: Some(0),
                    pendant_id: Some(0),
                    pb_reverse: Some(general_flags::PbReverse {
                        mobile_custom_font: Some(0),
                        pendant_diy_id: Some(0),
                        face_id: Some(0),
                        diy_font_timestamp: Some(0),
                        req_font_effect_id: Some(0),
                        subfont_id: Some(0),
                        vip_level: Some(0),
                        vip_type: Some(0),
                        user_bigclub_flag: Some(0),
                        user_bigclub_level: Some(0),
                        user_vip_info: Some(vec![]),
                        nameplate: Some(0),
                        nameplate_vip: Some(0),
                        gray_nameplate: Some(0),
                        unknown: Some(0),
                    }.encode_to_vec())
                }
            ))
        },
        Elem {
            aio_elem: Some(elem::AioElem::Flags2(
                ElemFlags2 {
                    color_text_id: Some(0)
                }
            ))
        },
    ];

    for cq in cqs {
        match convert_cq_to_elem(bot, contact, cq).await {
            Ok(elem) => elems.push(elem),
            Err(e) => {
                warn!("Failed to convert CQCode to Elem: {}", e);
            }
        }
    }

    RichText {
        attr: None,
        elems
    }
}

async fn convert_cq_to_elem(bot: &Arc<Bot>, contact: &Contact, cq: CQCode) -> Result<Elem, Error> {
    Ok(match cq {
        CQCode::Text(text) => Elem {
            aio_elem: Some(elem::AioElem::Text(
                Text {
                    text: text.to_string(),
                    ..Default::default()
                }
            ))
        },
        CQCode::At(at) => {
            let (nick, uid) = match contact {
                Contact::Group(_, gid) => get_group_member_info(bot, *gid, at.qq).await,
                _ => return Err(anyhow!("Unsupported AT: {}", at))
            }?;
            Elem {
                aio_elem: Some(elem::AioElem::Text(
                    Text {
                        text: format!("@{}", nick),
                        reversed: Some(TextReversed {
                            r#type: Some(if at.qq == 0 { 1 } else { 0 }),
                            target_uin: Some(at.qq),
                            flag: Some(0),
                            busi_type: Some(0),
                            target_uid: Some(uid),
                        }),
                        ..Default::default()
                    }
                ))
            }
        }
        _ => return Err(anyhow!("Unsupported CQCode: {}", cq.to_string()))
/*
        CQCode::Face(_) => {}
        CQCode::Image(_) => {}
        CQCode::BubbleFace(_) => {}
        CQCode::Reply(_) => {}
        CQCode::Record(_) => {}
        CQCode::Video(_) => {}
        CQCode::Basketball(_) => {}
        CQCode::NewRPS(_) => {}
        CQCode::NewDice(_) => {}
        CQCode::Poke(_) => {}
        CQCode::Touch(_) => {}
        CQCode::Music(_) => {}
        CQCode::Weather(_) => {}
        CQCode::Location(_) => {}
        CQCode::Share(_) => {}
        CQCode::Gift(_) => {}
        CQCode::CustomMusic(_) => {}*/
    })
}

async fn get_group_member_info(bot: &Arc<Bot>, group_id: i64, user_id: i64) -> anyhow::Result<(String, String)> {
    if user_id == 0 {
        Ok(("全体成员".to_string(), "0".to_string()))
    } else {
        let info = Bot::get_troop_member_card_info(bot, group_id, user_id, None).await?;
        let nick = if info.card_name.is_empty() {
            info.nickname
        } else {
            info.card_name
        };
        Ok((nick, info.uid))
    }
}