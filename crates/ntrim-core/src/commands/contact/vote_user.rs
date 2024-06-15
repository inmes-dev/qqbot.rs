use std::collections::HashMap;
use bytes::{Bytes, BytesMut};
use jcers::JcePut;
use ntrim_macros::command;
use crate::jce::friendlist::visitor_svc::{QQServiceReqHead, ReqFavorite};
use crate::jce::{next_request_id, pack_uni_request_data, RequestDataVersion3, RequestPacket};

struct SetProfileDetailCodec;

static COOKIE: &[u8; 10] = &[0x0Cu8, 0x18u8, 0x00u8, 0x01u8, 0x06u8, 0x01u8, 0x31u8, 0x16u8, 0x01u8, 0x35u8];

#[command("VisitorSvc.ReqFavorite", "vote_user", Service, Protobuf)]
impl SetProfileDetailCodec {
    async fn generate(
        bot: &Arc<Bot>,
        target: i64,
        count: i32,
        source: i32, // group member(friend) -> 2
        //cookies: Bytes
    ) -> Option<Vec<u8>> {
        if !std::env::var("ENABLE_VOTE").map_or(true, |v| v == "1") {
            return None;
        }
        let seq = next_request_id(bot.unique_id);
        let req = ReqFavorite {
            header: QQServiceReqHead {
                uin: bot.unique_id,
                sh_version: 1,
                seq,
                req_type: 1,
                triggered: 0,
                cookies: Bytes::from_static(COOKIE),
            },
            mid: target,
            op_type: 0,
            source,
            count,
            t: 0
        };
        let buf = RequestDataVersion3 {
            map: HashMap::from([(
                "ReqFavorite".to_string(),
                pack_uni_request_data(&req.freeze()),
            )]),
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "VisitorSvc".to_string(),
            s_func_name: "ReqFavorite".to_string(),
            s_buffer: buf.freeze(),
            i_request_id: seq,
            ..Default::default()
        };
        Some(pkt.freeze().to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<()> {
        None
    }
}