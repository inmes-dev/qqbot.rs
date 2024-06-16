use std::sync::Arc;
use anyhow::anyhow;
use ntrim_tools::cqp::CQCode;
use crate::await_response;
use crate::bot::Bot;
use crate::pb::msg::Grp;
use crate::pb::msg::send_msg_req::{C2c, RoutingHead};
use crate::service::msg::message_factory::convert_cq_to_msg;
use crate::servlet::olpush::msg::Contact;

impl Bot {
    pub async fn send_msg(self: &Arc<Bot>, contact: Contact, msg: Vec<CQCode>) -> anyhow::Result<u64> {
        let rich_text = convert_cq_to_msg(&contact, msg);
        let routing_head = convert_contact_to_routing_head(contact);

        let result = await_response!(tokio::time::Duration::from_secs(600), async {
            let receiver = Bot::send_raw_msg(self, routing_head, rich_text).await;
            if let Some(receiver) = receiver {
                receiver.await.map_err(|e| {
                    anyhow!("Failed to send message: {}", e)
                })
            } else {
                Err(anyhow!("Failed to send message: tcp connection error"))
            }
        }, |value| {
            Ok(value)
        }, |e| {
            Err(e)
        })?.ok_or(anyhow!("Failed to send message: timeout or wind ctrl"))?;

        return Ok(result);
    }
}

fn convert_contact_to_routing_head(contact: Contact) -> RoutingHead {
    match contact {
        Contact::Group(_, group_id) => {
            RoutingHead {
                grp: Some(Grp {
                    group_id,
                    ..Default::default()
                }),
                ..Default::default()
            }
        },
        Contact::Friend(_, _, uid) => {
            RoutingHead {
                c2c: Some(C2c {
                    uid,
                    ..Default::default()
                }),
                ..Default::default()
            }
        },
        Contact::Stranger(_, _, uid) => {
            RoutingHead {
                c2c: Some(C2c {
                    uid,
                    ..Default::default()
                }),
                ..Default::default()
            }
        },
    }
}