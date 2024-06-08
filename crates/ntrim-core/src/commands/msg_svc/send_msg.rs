use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::AtomicU32;
use chrono::Local;
use log::info;
use once_cell::sync::Lazy;
use prost::Message;
use rand::{Rng, RngCore, thread_rng};
use ntrim_macros::command;
use crate::pb::msg::send_msg_req::{ContentHead, RoutingHead};
use crate::pb::msg::{MessageBody, RichText, SendMsgReq};
use crate::pb::trpc::status::{SilenceState, SsoHeartBeatRequest, SsoHeartBeatResponse};

struct SendMsgCodec;

#[command("MessageSvc.PbSendMsg", "send_msg", Protobuf, Service)]
impl SendMsgCodec {
    async fn generate(
        bot: &Arc<Bot>,
        contact: RoutingHead,
        rich_text: RichText
    ) -> Option<Vec<u8>> {
        let send_msg = SendMsgReq {
            routing_head: contact,
            content_head: ContentHead {
                pkg_num: 1,
                pkg_index: 0,
                div_seq: 0
            },
            msg_body: MessageBody {
                rich_text: Some(rich_text)
            },
            msg_seq: next_msg_seq(bot.unique_id) as u64,
            msg_time: thread_rng().gen_range(1700000000 .. 3100000000),
            via: 0
        };
        //info!("Generated a message: {:?}", hex::encode(&send_msg.encode_to_vec()));
        Some(send_msg.encode_to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<u64> {
        info!("Received a message from the server: {:?}", hex::encode(&data));
        None
    }
}

fn next_msg_seq(uin: u64) -> u32 {
    static MAP_SEQ: Lazy<Mutex<HashMap<u64, AtomicU32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
    let mut map_seq = MAP_SEQ.lock().unwrap();
    let seq = map_seq.entry(uin).or_insert(AtomicU32::new(17050));
    if seq.load(std::sync::atomic::Ordering::Relaxed) >= 0x8000 {
        seq.store(17050, std::sync::atomic::Ordering::Relaxed);
    }
    seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}