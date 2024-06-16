// group_member_card.get_group_member_card_info

use prost::Message;
use ntrim_macros::command;
use crate::commands::troop::GroupMemberPermission;
use crate::commands::troop::GroupInfo;
use crate::pb;
use crate::pb::get_group_member_card_info::{ * };
use crate::pb::im::honor::GroupUserCardHonor;

struct GetTroopMemberCardInfoCodec;

#[command("group_member_card.get_group_member_card_info", "_get_group_member_card_info", Service, Protobuf)]
impl GetTroopMemberCardInfoCodec {
    async fn generate(bot: &Arc<Bot>, group_code: i64, target: i64) -> Option<Vec<u8>> {
        Some(GroupMemberReqBody {
            group_code,
            uin: target,
            new_client: true,
            client_type: 1,
            rich_card_name_ver: 1
        }.encode_to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<crate::commands::troop::GroupMemberInfo> {
        let info = GroupMemberRspBody::decode(data.as_slice()).ok()?;
        let mem_info = info.mem_info?;
        let honor = GroupUserCardHonor::decode(mem_info.honor.as_bytes()).map_or(vec![], |value| {
            value.id
        });
        //let remark = unsafe { String::from_utf8_unchecked(mem_info.remark) };
        let nick = unsafe { String::from_utf8_unchecked(mem_info.nick) };
        let card = unsafe { String::from_utf8_unchecked(mem_info.card) };
        let area = unsafe { String::from_utf8_unchecked(mem_info.location) };
        let title = unsafe { String::from_utf8_unchecked(mem_info.special_title) };

        Some(crate::commands::troop::GroupMemberInfo {
            uin: mem_info.uin,
            gender: match mem_info.sex {
                255 => -1,
                0 => 0,
                1 => 1,
                _ => -1
            },
            nickname: nick,
            card_name: card,
            level: mem_info.level as i16,
            join_time: mem_info.join,
            last_speak_time: mem_info.last_speak,
            special_title: title,
            special_title_expire_time: mem_info.special_title_expire_time as i64,
            permission: match mem_info.role {
                3 => GroupMemberPermission::Owner,
                2 => GroupMemberPermission::Administrator,
                _ => GroupMemberPermission::Member,
            },
            honor,
            distance: mem_info.int64_distance,
            area,
            ..Default::default()
        })
    }
}