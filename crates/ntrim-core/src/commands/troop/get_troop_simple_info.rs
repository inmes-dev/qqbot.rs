use std::collections::HashMap;
use anyhow::Error;
use bytes::{Buf, Bytes};
use jcers::JcePut;
use ntrim_macros::command;
use crate::{await_response, db, jce};
use crate::db::PG_POOL;
use crate::jce::{next_request_id, pack_uni_request_data};
use crate::jce::friendlist::get_multi_group_info::{TroopInfoV2, TroopMultiInfoRequest};

struct GetTroopMultiInfoCodec;

// 非常垃圾没什么鸟用的api，如果没需要，无需使用！
#[command("friendlist.GetMultiTroopInfoReq", "get_multi_troop_simple_info", Service)]
impl GetTroopMultiInfoCodec {
    async fn generate(bot: &Arc<Bot>, group_list: Vec<i64>) -> Option<Vec<u8>> {
        let req = TroopMultiInfoRequest {
            uin: bot.unique_id,
            group_list,
            rich_info: 1
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([(
                "GMTIREQ".to_string(),
                pack_uni_request_data(&req.freeze()),
            )]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            c_packet_type: 0x00,
            i_message_type: 0,
            i_request_id: next_request_id(bot.unique_id),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetMultiTroopInfoReq".to_string(),
            s_buffer: buf.freeze(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        };
        Some(pkt.freeze().to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<Vec<crate::db::GroupInfo>> {
        let mut payload = Bytes::from(data);
        let mut request: Result<jce::RequestPacket, _> = jcers::from_buf(&mut payload).map_err(Error::from);
        if request.is_err() {
            error!("Failed to parse RequestPacket for GetTroopMultiInfo: {:?}", request.unwrap_err());
            return None;
        }
        let mut request = request.unwrap();
        let mut data: Result<jce::RequestDataVersion3, _> =
            jcers::from_buf(&mut request.s_buffer).map_err(Error::from);
        if data.is_err() {
            error!("Failed to parse GetTroopMultiInfo: {:?}", data.unwrap_err());
            return None;
        }
        let mut data = data.unwrap();
        let mut fl_resp = data.map.remove("GMTIRESP").ok_or_else(|| {
            Error::msg("decode_group_multi_info_response GetTroopMultiInfo not found")
        });
        if fl_resp.is_err() {
            error!("Failed to get GetTroopMultiInfo: {:?}", fl_resp.unwrap_err());
            return None;
        }
        let mut fl_resp = fl_resp.unwrap();
        fl_resp.advance(1);
        let mut r = jcers::Jce::new(&mut fl_resp);
        let groups: Result<Vec<TroopInfoV2>, _> = r.get_by_tag(3).map_err(Error::from);
        if groups.is_err() {
            error!("Failed to get groups in GetTroopMultiInfo: {:?}", groups.unwrap_err());
            return None;
        }
        let groups = groups.unwrap();
        let l = groups
            .into_iter()
            .map(|g| crate::db::GroupInfo {
                uin: g.group_uin,
                code: g.group_code,
                name: g.group_name,
                memo: g.group_memo,
                owner_uin: g.group_owner,
                member_count: g.member_num,
                ..Default::default()
            })
            .collect();
        Some(l)
    }
}
