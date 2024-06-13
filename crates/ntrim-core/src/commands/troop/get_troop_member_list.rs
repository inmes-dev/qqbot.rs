use std::collections::HashMap;
use anyhow::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use jcers::{Jce, JcePut};
use prost::Message;
use serde::Serialize;
#[cfg(feature = "sql")]
use sqlx::FromRow;
use url::quirks::host;
use ntrim_macros::command;
use ntrim_tools::oicq::group_code2uin;
#[cfg(feature = "sql")]
use crate::db::{GroupInfo, PG_POOL};
#[cfg(feature = "sql")]
use crate::db;
use crate::{await_response, jce};
use crate::jce::{next_request_id, pack_uni_request_data};
use crate::jce::friendlist::get_troop_member_list::{TroopMemberInfo, TroopMemberListRequest};
use crate::pb::im::honor::GroupUserCardHonor;

struct GetTroopMemberListCodec;

#[derive(Debug, Default, Clone, Serialize)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
pub struct GroupMemberInfo {
    pub group_code: i64,
    pub uin: i64,
    pub gender: i16,
    pub nickname: String,
    pub card_name: String,
    pub level: i16,
    pub join_time: i64,
    pub last_speak_time: i64,
    pub special_title: String,
    pub special_title_expire_time: i64,
    pub shut_up_timestamp: i64,
    pub permission: GroupMemberPermission,
    pub uid: String,
    pub area: String,
    pub distance: i64,
    pub honor: Vec<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[repr(i32)]
pub enum GroupMemberPermission {
    Owner = 1,
    Administrator = 2,
    #[default]
    Member = 3,
}

#[command("friendlist.getTroopMemberList", "_get_troop_member_list", Service)]
impl GetTroopMemberListCodec {
    async fn generate(bot: &Arc<Bot>, group_code: i64, next_uin: i64) -> Option<Vec<u8>> {
        let payload = TroopMemberListRequest {
            uin: bot.unique_id,
            group_code,
            next_uin,
            group_uin: group_code2uin(group_code),
            version: 3,
            req_type: 1,
            get_list_appoint_time: 0,
            rich_card_name_ver: 1,
            uid_ver: 1,
        };
        let mut b = BytesMut::new();
        b.put_u8(0x0A);
        b.put_slice(&payload.freeze());
        b.put_u8(0x0B);
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([("GTML".to_string(), b.into())]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            i_request_id: next_request_id(bot.unique_id),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetTroopMemberListReq".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        Some(pkt.freeze().to_vec())
    }

    async fn decode_troop_list_member(data: Vec<u8>) -> Result<(i64, Vec<GroupMemberInfo>), Error> {
        let mut payload = Bytes::from(data);
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(Error::from)?;

        let mut data: jce::RequestDataVersion3 =
            jcers::from_buf(&mut request.s_buffer).map_err(Error::from)?;

        let mut fl_resp = data.map.remove("GTMLRESP").ok_or_else(|| {
            Error::msg("decode_group_member_list_response GTMLRESP not found")
        })?;

        fl_resp.advance(1);
        let mut r = Jce::new(&mut fl_resp);
        let members: Vec<TroopMemberInfo> = r.get_by_tag(3)
            .map_err(|e|
                Error::msg(format!("decode_group_members failed: {}", e))
            )?;
        let next_uin = r.get_by_tag(4).map_err(Error::from)?;
        let mut l = Vec::new();
        for m in members {
            let honor = GroupUserCardHonor::decode(m.group_honor).map_or(vec![], |value| {
                value.id
            });
            l.push(GroupMemberInfo {
                uin: m.member_uin,
                gender: match m.gender {
                    255 => -1,
                    0 => 0,
                    1 => 1,
                    _ => -1
                },
                nickname: m.nick,
                card_name: m.name,
                level: m.member_level as i16,
                join_time: m.join_time,
                last_speak_time: m.last_speak_time,
                special_title: m.special_title,
                special_title_expire_time: m.special_title_expire_time,
                shut_up_timestamp: m.shut_up_timestap,
                permission: match m.flag {
                    1 => GroupMemberPermission::Administrator,
                    _ => GroupMemberPermission::Member,
                },
                uid: m.uid,
                honor,
                ..Default::default()
            })
        }
        Ok((next_uin, l))
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<(Vec<GroupMemberInfo>, i64)> {
        return match Self::decode_troop_list_member(data).await {
            Ok((next_uin, members)) => Some((members, next_uin)),
            Err(e) => {
                error!("Failed to parse GetTroopMemberListResp: {:?}", e);
                None
            }
        };
    }
}

impl Bot {
    pub async fn get_troop_member_list(
        self: &Arc<Self>,
        group_id: i64,
        owner_uin: i64,
    ) -> Result<Vec<GroupMemberInfo>, Error> {
        let mut next_uin = 0;
        let mut list = Vec::new();
        loop {
            match await_response!(tokio::time::Duration::from_secs(10), async {
                let rx = Bot::_get_troop_member_list(self, group_id, next_uin).await;
                if let Some(rx) = rx {
                    rx.await.map_err(|e| Error::new(e))
                } else {
                    Err(Error::msg("Unable to get_troop_member_list: tcp connection exception"))
                }
            }, |value| {
                Ok(value)
            }, |e| {
                Err(e)
            }) {
                Ok(Some((mut rl, n))) => {
                    if rl.is_empty() {
                        return Err(Error::msg("GroupMemberListResponse.list"));
                    }
                    for m in rl.iter_mut() {
                        if m.uin == owner_uin {
                            m.permission = GroupMemberPermission::Owner;
                        }
                        m.group_code = group_id;
                    }
                    list.append(&mut rl);
                    next_uin = n;
                    if next_uin == 0 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(None) => {
                    warn!("get_troop_member_list: no more data");
                    break;
                }
            }
        }
        #[cfg(feature = "sql")]
        if db::is_initialized() {
            let list = list.clone();
            tokio::spawn(async move {
                let pool = PG_POOL.get().unwrap();
                for member in list.into_iter() {
                    GroupMemberInfo::insert(pool, member).await
                        .expect("Failed to insert group member info");
                }
            });
        }
        Ok(list)
    }

    #[cfg(feature = "sql")]
    pub async fn get_troop_member_list_from_cache(
        self: &Arc<Self>,
        group_id: i64
    ) -> Result<Vec<GroupMemberInfo>, Error> {
        let pool = PG_POOL.get().unwrap();
        GroupMemberInfo::query_by_group_id(pool, group_id).await
    }
}