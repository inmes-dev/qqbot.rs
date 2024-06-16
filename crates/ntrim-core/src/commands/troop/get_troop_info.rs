use prost::Message;
use ntrim_macros::command;
use crate::{*};
use crate::commands::troop::get_troop_list::GroupInfo;
use crate::pb::oidb::{D88dGroupExInfoOnly, D88dGroupHeadPortrait, D88dGroupInfo, D88dReqBody, D88dRspBody, ReqGroupInfo};

struct GetTroopInfoCodec;

#[command("OidbSvc.0x88d_0", "_get_troop_info", Service, Protobuf)]
impl GetTroopInfoCodec {
    async fn generate(bot: &Arc<Bot>, group_codes: Vec<i64>) -> Option<Vec<u8>> {
        let body = D88dReqBody {
            app_id: Some(181986905),
            req_group_info: group_codes
                .into_iter()
                .map(|group_code| ReqGroupInfo {
                    group_code: Some(group_code as u64),
                    stgroupinfo: Some(D88dGroupInfo {
                        group_owner: Some(0),
                        group_uin: Some(0),
                        group_create_time: Some(0),
                        group_flag: Some(0),
                        group_member_max_num: Some(0),
                        group_member_num: Some(0),
                        group_option: Some(0),
                        group_level: Some(0),
                        group_face: Some(0),
                        group_name: Some(vec![]),
                        group_memo: Some(vec![]),
                        group_finger_memo: Some(vec![]),
                        group_last_msg_time: Some(0),
                        group_cur_msg_seq: Some(0),
                        group_question: Some(vec![]),
                        group_answer: Some(vec![]),
                        group_grade: Some(0),
                        active_member_num: Some(0),
                        head_portrait_seq: Some(0),
                        msg_head_portrait: Some(D88dGroupHeadPortrait::default()),
                        st_group_ex_info: Some(D88dGroupExInfoOnly::default()),
                        group_sec_level: Some(0),
                        cmduin_privilege: Some(0),
                        no_finger_open_flag: Some(0),
                        no_code_finger_open_flag: Some(0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .collect(),
            pc_client_version: Some(0)
        };
       oidb_request!(0x88d, 0, body.encode_to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<Vec<GroupInfo>> {
        let data = oidb_response!(0x88d, 0, data.as_slice())?;
        let groups = D88dRspBody::decode(data.as_slice()).ok()?.rsp_group_info;
        Some(groups
            .into_iter()
            .filter_map(|g| {
                let code = g.group_code? as i64;
                let info = g.group_info?;
                Some(GroupInfo {
                    uin: info.group_uin? as i64,
                    code,
                    name: String::from_utf8_lossy(&info.group_name?).into_owned(),
                    memo: String::from_utf8_lossy(&info.group_memo?).into_owned(),
                    owner_uin: info.group_owner? as i64,
                    group_create_time: info.group_create_time.unwrap_or(0),
                    group_level: info.group_level.unwrap_or_default(),
                    member_count: info.group_member_num.unwrap_or_default(),
                    max_member_count: info.group_member_max_num.unwrap_or_default(),
                    shut_up_timestamp: info.shutup_timestamp.unwrap_or_default() as i64,
                    my_shut_up_timestamp: info.shutup_timestamp_me.unwrap_or_default() as i64,
                    last_msg_seq: info.group_cur_msg_seq.unwrap_or_default() as i64,
                })
            })
            .collect())
    }
}

