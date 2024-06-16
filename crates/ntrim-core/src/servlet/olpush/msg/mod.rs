pub mod decoder;
pub mod encoder;
pub mod record;

use std::sync::Arc;
use chrono::{Local, NaiveDateTime};
use log::warn;
use prost::Message as ProstMessage;
use crate::bot::Bot;
use crate::pb::msg::{Grp, olpush_routing_head};
use crate::pb::trpc::olpush::Message;
pub use record::{ * };

pub(super) async fn on_group_msg(bot: Arc<Bot>, msg: Message) {
    let msg_time = msg.content_head.msg_time as i64;
    let msg_seq = msg.content_head.msg_seq;
    let msg_uid = msg.content_head.msg_uid;
    let (sender_uid, sender_uin) = (msg.routing_head.peer_uid.unwrap(), msg.routing_head.peer_id);
    let from_sub_appid = msg.routing_head.from_app_id;
    let platform = msg.routing_head.platform;
    let (group_id, sender_nick, group_name) = match msg.routing_head.contact {
        Some(olpush_routing_head::Contact::Grp(grp)) => (
            grp.group_id,
            grp.sender_nick.map_or_else(|| "".to_string(), |x| x),
            grp.group_name.map_or_else(|| "".to_string(), |x| x)
        ),
        _ => {
            warn!("Invalid routing_head, msg_seq: {}", msg_seq);
            return;
        }
    };

    if msg.msg_body.rich_text.is_none() {
        warn!("Empty rich_text, msg_seq: {}", msg_seq);
        return;
    }

    let mut record = MessageRecord {
        contact: Contact::Group(group_name, group_id),
        sender_id: sender_uin,
        sender_uid,
        sender_nick,
        sender_unique_title: "".to_string(),
        msg_time,
        msg_seq,
        msg_uid,
        elements: Vec::new(),
    };

    let mut rich_text = msg.msg_body.rich_text.unwrap();

    #[cfg(feature = "sql")]
    if crate::db::is_initialized() {
        let pool = crate::db::PG_POOL.get().unwrap();
        MessageRecord::insert(pool, &bot, &record, rich_text.encode_to_vec()).await.map_err(|e| {
            warn!("Failed to insert message to pgsql: {:?}", e);
        }).unwrap();
    }

    decoder::parse_elements(&bot, &mut record, rich_text.elems).await;

    /*let raw_msg = record.elements.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    if raw_msg == "ping" {
        let contact = crate::pb::msg::send_msg_req::RoutingHead {
            c2c: None,
            grp: Some(Grp {
                group_id,
                ..Default::default()
            }),
        };
        Bot::send_msg(&bot, contact, crate::pb::msg::RichText {
            attr: None,
            elems: vec![

                crate::pb::msg::Elem {
                    aio_elem: Some(crate::pb::msg::elem::AioElem::Text(
                        crate::pb::msg::Text {
                            text: format!("pong, receive_time: {}, time: {}", msg_time, Local::now().timestamp()).to_string(),
                            ..Default::default()
                        }
                    ))
                },
            ]
        }).await;
    }*/

    println!("{}", record);
}