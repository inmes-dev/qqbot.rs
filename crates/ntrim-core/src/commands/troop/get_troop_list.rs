use std::collections::HashMap;
use anyhow::Error;
use bytes::{Buf, Bytes};
use jcers::JcePut;
use serde::Serialize;
use ntrim_macros::command;
use crate::{*};

use crate::jce::{next_request_id, pack_uni_request_data};
use crate::jce::friendlist::get_troop_list::TroopNumber;

struct GetTroopListCodec;

#[derive(Debug, Default, Clone, Serialize)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
pub struct GroupInfo {
    #[serde(rename="group_uin")]
    pub uin: i64,
    #[cfg_attr(feature = "sql", sqlx(rename = "id"))]
    #[serde(rename="group_id")]
    pub code: i64,
    #[serde(rename="group_name")]
    pub name: String,
    #[serde(rename="group_memo")]
    pub memo: String,
    #[cfg_attr(feature = "sql", sqlx(rename = "owner"))]
    #[serde(rename="group_owner")]
    pub owner_uin: i64,
    #[cfg_attr(feature = "sql", sqlx(rename = "create_time"))]
    pub group_create_time: i64,
    #[cfg_attr(feature = "sql", sqlx(rename = "level"))]
    pub group_level: i32,
    pub member_count: i32,
    pub max_member_count: i32,
    // 全群禁言时间
    pub shut_up_timestamp: i64,
    // 自己被禁言时间
    pub my_shut_up_timestamp: i64,
    // 最后一条信息的SEQ,只有通过 GetGroupInfo 函数获取的 GroupInfo 才会有
    pub last_msg_seq: i64,
}

#[command("friendlist.GetTroopListReqV2", "_get_troop_list", Service)]
impl GetTroopListCodec {
    async fn generate(bot: &Arc<Bot>, cookies: &Bytes) -> Option<Vec<u8>> {
        let req = jce::friendlist::get_troop_list::TroopListRequest {
            uin: bot.unique_id,
            get_msf_msg_flag: 1,
            cookies: Bytes::clone(cookies),
            group_info: vec![],
            group_flag_ext: 1,
            version: 7,
            company_id: 0,
            version_num: 1,
            get_long_group_name: 1,
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([(
                "GetTroopListReqV2Simplify".to_string(),
                pack_uni_request_data(&req.freeze()),
            )]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            c_packet_type: 0x00,
            i_message_type: 0,
            i_request_id: next_request_id(bot.unique_id),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetTroopListReqV2Simplify".to_string(),
            s_buffer: buf.freeze(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        };
        Some(pkt.freeze().to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<(Vec<GroupInfo>, Bytes)> {
        //info!("Parsing GetTroopListRespV2: {}", hex::encode(&data));
        let mut payload = Bytes::from(data);
        let mut request: Result<jce::RequestPacket, _> = jcers::from_buf(&mut payload).map_err(Error::from);
        // 改成match会好看一点？没好看到什么地方去...
        if request.is_err() {
            error!("Failed to parse RequestPacket for GetTroopListRespV2: {:?}", request.unwrap_err());
            return None;
        }
        let mut request = request.unwrap();
        let mut data: Result<jce::RequestDataVersion3, _> =
            jcers::from_buf(&mut request.s_buffer).map_err(Error::from);
        if data.is_err() {
            error!("Failed to parse GetTroopListRespV2: {:?}", data.unwrap_err());
            return None;
        }
        let mut data = data.unwrap();
        let mut fl_resp = data.map.remove("GetTroopListRespV2").ok_or_else(|| {
            Error::msg("decode_group_list_response GetTroopListRespV2 not found")
        });
        if fl_resp.is_err() {
            error!("Failed to get GetTroopListRespV2: {:?}", fl_resp.unwrap_err());
            return None;
        }
        let mut fl_resp = fl_resp.unwrap();
        fl_resp.advance(1);
        let mut r = jcers::Jce::new(&mut fl_resp);
        let vec_cookie: Result<Bytes, _> = r.get_by_tag(4).map_err(Error::from);
        if vec_cookie.is_err() {
            error!("Failed to get cookies in GetTroopListRespV2: {:?}", vec_cookie.unwrap_err());
            return None;
        }
        let groups: Result<Vec<TroopNumber>, _> = r.get_by_tag(5).map_err(Error::from);
        if groups.is_err() {
            error!("Failed to get groups in GetTroopListRespV2: {:?}", groups.unwrap_err());
            return None;
        }
        let groups = groups.unwrap();
        let l = groups
            .into_iter()
            .map(|g| GroupInfo {
                uin: g.group_uin,
                code: g.group_code,
                name: g.group_name,
                memo: g.group_memo,
                owner_uin: g.group_owner_uin,
                member_count: g.member_num as i32,
                max_member_count: g.max_group_member_num as i32,
                shut_up_timestamp: g.shut_up_timestamp,
                my_shut_up_timestamp: g.my_shut_up_timestamp,
                ..Default::default()
            })
            .collect();
        Some((l, vec_cookie.unwrap()))
    }
}
