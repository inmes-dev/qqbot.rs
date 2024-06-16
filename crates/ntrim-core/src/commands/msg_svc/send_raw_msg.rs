use std::collections::HashMap;
use std::sync::{Mutex};
use std::sync::atomic::AtomicU32;
use bytes::Bytes;
use once_cell::sync::Lazy;
use prost::Message;
use rand::{Rng, thread_rng};
use ntrim_macros::command;
use crate::pb::msg::send_msg_req::{ContentHead, RoutingHead};
use crate::pb::msg::{MessageBody, RichText, SendMsgReq, SendMsgRsp};

struct SendMsgCodec;

#[command("MessageSvc.PbSendMsg", "send_raw_msg", Protobuf, Service)]
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
        //info!("Sent message successfully, seq: {}", hex::encode(&data));
        let data = Bytes::from(data);
        let send_msg = SendMsgRsp::decode(data.as_ref()).ok()?;
        if send_msg.result != 0 {
            error!("Failed to send message, code: {}", send_msg.result);
            return None;
        }
        //if send_msg.msg_seq.is_none() {
        //    error!("Failed to send message, reason: account is under wind control");
        //    return None;
        //}
        return Some(send_msg.msg_seq.unwrap_or(0));
    }
}

fn next_msg_seq(uin: i64) -> u32 {
    static MAP_SEQ: Lazy<Mutex<HashMap<i64, AtomicU32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
    let mut map_seq = MAP_SEQ.lock().unwrap();
    let seq = map_seq.entry(uin).or_insert(AtomicU32::new(17050));
    if seq.load(std::sync::atomic::Ordering::Relaxed) >= 0x8000 {
        seq.store(17050, std::sync::atomic::Ordering::Relaxed);
    }
    seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}