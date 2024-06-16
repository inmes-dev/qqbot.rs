use anyhow::{anyhow, Error};
use log::warn;
use prost::Message as _;
use ntrim_tools::cqp::CQCode;
use crate::Contact;
use crate::pb::msg::{ * };

pub(crate) fn convert_cq_to_msg(contact: &Contact, cqs: Vec<CQCode>) -> RichText {
    let mut elems = vec![
        Elem {
            aio_elem: Some(elem::AioElem::GeneralFlags(
                GeneralFlags {
                    bubble_diy_text_id: Some(0),
                    bubble_sub_id: Some(0),
                    pendant_id: Some(0),
                    pb_reverse: Some(crate::pb::msg::general_flags::PbReverse {
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
        match convert_cq_to_elem(contact, cq) {
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

fn convert_cq_to_elem(contact: &Contact, cq: CQCode) -> Result<Elem, Error> {
    Ok(match cq {
        CQCode::Text(text) => Elem {
            aio_elem: Some(elem::AioElem::Text(
                Text {
                    text: text.to_string(),
                    ..Default::default()
                }
            ))
        },
/*        CQCode::At(at) => {

        }*/
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