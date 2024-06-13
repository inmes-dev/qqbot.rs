use std::collections::HashMap;
use anyhow::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use jcers::JcePut;
use log::info;
use prost::Message;
use serde::Serialize;
use ntrim_macros::command;
use crate::{oidb_request, oidb_response};
use crate::jce::{next_request_id, pack_uni_request_data, RequestPacket};
use crate::jce::summary::{MCardSvc, RespSummaryCard, RespSummaryCardHead};
use crate::pb::summary_card;

struct GetSummaryCardCodec;

#[derive(Debug, Default, Clone)]
pub struct SummaryCardInfo {
    pub uin: i64,
    pub sex: u8,
    pub age: u8,
    pub nickname: String,
    pub level: i32,
    pub city: String,
    pub sign: String,
    pub mobile: String,
    pub login_days: i64,
    /// 用于点赞
    pub cookie: Bytes,
}

#[command("SummaryCard.ReqSummaryCard", "get_summary_card", Service)]
impl GetSummaryCardCodec {
    async fn generate(bot: &Arc<Bot>, target: i64) -> Option<Vec<u8>> {
        let gate = summary_card::GateVaProfileGateReq {
            u_cmd: Some(3),
            st_privilege_req: Some(summary_card::GatePrivilegeBaseInfoReq {
                u_req_uin: Some(target),
            }),
            st_gift_req: Some(summary_card::GateGetGiftListReq {
                uin: Some(target as i32),
            }),
            st_vip_care: Some(summary_card::GateGetVipCareReq { uin: Some(target) }),
            oidb_flag: vec![
                summary_card::GateOidbFlagInfo {
                    fieled: Some(42334),
                    byets_value: None,
                },
                summary_card::GateOidbFlagInfo {
                    fieled: Some(42340),
                    byets_value: None,
                },
                summary_card::GateOidbFlagInfo {
                    fieled: Some(42344),
                    byets_value: None,
                },
                summary_card::GateOidbFlagInfo {
                    fieled: Some(42354),
                    byets_value: None,
                },
            ],
            ..Default::default()
        }.encode_to_vec();
        let trpc = &bot.client;
        let session = trpc.session.read().await;
        let seq = session.next_seq();
        let qq_ver = session.protocol.build_ver.clone();
        let apk_ver = session.protocol.apk_ver.clone();

        std::mem::drop(session);

        let business_buf = {
            let mut w = BytesMut::new();
            let comm = summary_card::BusiComm {
                ver: Some(1),
                seq: Some(seq as i32),
                fromuin: Some(bot.unique_id),
                touin: Some(target),
                service: Some(16),
                platform: Some(2),
                qqver: Some(qq_ver),
                build: Some(4945),
                ..Default::default()
            }.encode_to_vec();
            w.put_u8(40);
            w.put_u32(comm.len() as u32);
            w.put_u32(gate.len() as u32);
            w.put_slice(&comm);
            w.put_slice(&gate);
            w.put_u8(42);
            w.freeze()
        };

        let mcard_svc_buf = {
            let mut pbuf = BytesMut::new();
            let req = MCardSvc {
                ver: 1,
                uin: bot.unique_id,
                target,
                qq_ver: apk_ver,
                come_from: 31,
                plat: 109,
            };
            let buf = crate::jce::RequestDataVersion3 {
                map: HashMap::from([
                    ("req".to_string(), pack_uni_request_data(&req.freeze())),
                ]),
            };
            let pkt = RequestPacket {
                i_version: 3,
                c_packet_type: 0,
                i_request_id: 0,
                s_servant_name: "MCardSvc".to_string(),
                s_func_name: "query".to_string(),
                s_buffer: buf.freeze(),
                ..Default::default()
            };
            let vec = pkt.freeze();
            pbuf.put_i32(vec.len() as i32);
            pbuf.put(vec);
            pbuf.freeze()
        };

        let req = crate::jce::summary::SummaryCardReq {
            uin: target,
            come_from: 31,
            seed: Bytes::from(vec![0]),
            search_name: "".to_string(),
            get_control: 69181,
            add_friend_source: 3001,
            secure_sig: Bytes::from(vec![0]),
            req_tmp_info: mcard_svc_buf,
            req_medal_wall_info: 0,
            req_0x5eb_field_id: vec![
                27225, 27224, 42122, 42121, 27236, 27238, 42167, 42172, 40324, 42284, 42326, 42325,
                42356, 42363, 42361, 42367, 42377, 42425, 42505, 42488,
            ],
            req_services: vec![business_buf],
            req_nearby_god_info: 1,
            req_extend_card: 1,
            u24: 1,
            ..Default::default()
        };
        let mut head = jcers::JceMut::new();
        head.put_i32(2, 0);
        let buf = crate::jce::RequestDataVersion3 {
            map: HashMap::from([
                ("ReqHead".to_string(), pack_uni_request_data(&head.freeze())),
                ("ReqSummaryCard".to_string(), pack_uni_request_data(&req.freeze()), ),
            ]),
        };
        let pkt = RequestPacket {
            i_version: 2,
            c_packet_type: 0,
            i_request_id: 0,
            s_servant_name: "SummaryCardServantObj".to_string(),
            s_func_name: "ReqSummaryCard".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };

        let data = pkt.freeze().encode_to_vec();
        //info!("get_summary_card: {:?}", hex::encode(&data));

        Some(data)
    }

    fn decode(data: Vec<u8>) -> Result<SummaryCardInfo, Error> {
        let mut payload = Bytes::from(data);
        let mut request: RequestPacket =
            jcers::from_buf(&mut payload).map_err(Error::from)?;
        let mut data: crate::jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(Error::from)?;
        let mut head = data
            .map
            .remove("RespHead")
            .ok_or_else(|| Error::msg("missing RespHead"))?
            .remove("SummaryCard.RespHead")
            .ok_or_else(|| Error::msg("missing SummaryCard.RespHead"))?;
        head.advance(1);
        let head: RespSummaryCardHead = jcers::from_buf(&mut head)?;
        let mut rsp = data
            .map
            .remove("RespSummaryCard")
            .ok_or_else(|| Error::msg("missing RespSummaryCard"))?
            .remove("SummaryCard_Old.RespSummaryCard")
            .ok_or_else(|| Error::msg("missing SummaryCard_Old.RespSummaryCard"))?;
        rsp.advance(1);
        let rsp: RespSummaryCard = jcers::from_buf(&mut rsp)?;
        let info = SummaryCardInfo {
            sex: rsp.sex,
            age: rsp.age,
            nickname: rsp.nickname,
            level: rsp.level,
            city: rsp.city,
            sign: rsp.sign,
            mobile: rsp.mobile,
            uin: rsp.uin,
            login_days: rsp.login_days,
            cookie: head.cookie,
        };
        // TODO more info
        Ok(info)
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<SummaryCardInfo> {
        match Self::decode(data) {
            Ok(info) => Some(info),
            Err(e) => {
                error!("parse summary card failed: {:?}", e);
                None
            }
        }
    }
}