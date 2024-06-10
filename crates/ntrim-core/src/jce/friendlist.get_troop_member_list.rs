use jcers::{JceGet, JcePut};
use bytes::Bytes;

jce_struct!(TroopMemberListRequest {
    0 => uin: i64,
    1 => group_code: i64,
    2 => next_uin: i64,
    3 => group_uin: i64,
    4 => version: i64,
    5 => req_type: i64,
    6 => get_list_appoint_time: i64,
    7 => rich_card_name_ver: u8,
    8 => uid_ver: u8,
});

jce_struct!(TroopMemberInfo {
    0 => member_uin: i64, // uin
    1 => face_id: i16,
    2 => age: u8, // age
    3 => gender: u8, // -1 unknown 1 female 0 male
    4 => nick: String, // nickname
    5 => status: u8, // status
    6 => show_name: String,
    8 => name: String,
    12 => memo: String,
    13 => auto_remark: String, // remark
    14 => member_level: i64,
    15 => join_time: i64, // sec
    16 => last_speak_time: i64, // sec
    17 => credit_level: i64,
    18 => flag: i64, // owner 0 admin 1 member 0
    19 => flag_ext: i64,
    20 => point: i64,
    21 => concerned: u8,
    22 => shielded: u8,
    23 => special_title: String, // title
    24 => special_title_expire_time: i64,
    25 => job: String,
    26 => apollo_flag: u8,
    27 => apollo_timestamp: i64,
    28 => global_group_level: i64,
    29 => title_id: i64,
    30 => shut_up_timestap: i64,
    31 => global_group_point: i64,
    33 => rich_card_name_ver: u8,
    34 => vip_type: i64,
    35 => vip_level: i64,
    36 => big_club_level: i64,
    37 => big_club_flag: i64,
    38 => nameplate: i64,
    39 => group_honor: Bytes,
    40 => vec_name: Bytes, // group_nick
    41 => rich_flag: u8, // 20
    42 => uid: String, // qq uid
});

jce_struct!(FriendInfo {
    0 => friend_uin: i64,
    1 => group_id: u8,
    2 => face_id: i16,
    3 => remark: String,
    4 => qq_type: u8,
    5 => status: u8,
    6 => member_level: u8,
    7 => is_mqq_online: u8,
    8 => qq_online_state: u8,
    9 => is_iphone_online: u8,
    10 => detail_status_flag: u8,
    11 => qq_online_state_v2: u8,
    12 => show_name: String,
    13 => is_remark: u8,
    14 => nick: String,
    15 => special_flag: u8,
    16 => im_group_id: Bytes,
    17 => msf_group_id: Bytes,
    18 => term_type: i32,
    20 => network: u8,
    21 => ring: Bytes,
    22 => abi_flag: i64,
    23 => face_addon_id: i64,
    24 => network_type: i32,
    25 => vip_font: i64,
    26 => icon_type: i32,
    27 => term_desc: String,
    28 => color_ring: i64,
    29 => apollo_flag: u8,
    30 => apollo_timestamp: i64,
    31 => sex: u8,
    32 => founder_font: i64,
    33 => eim_id: String,
    34 => eim_mobile: String,
    35 => olympic_torch: u8,
    36 => apollo_sign_time: i64,
    37 => lavi_uin: i64,
    38 => tag_update_time: i64,
    39 => game_last_login_time: i64,
    40 => game_app_id: i64,
    41 => card_id: Bytes,
    42 => bit_set: i64,
    43 => king_of_glory_flag: u8,
    44 => king_of_glory_rank: i64,
    45 => master_uin: String,
    46 => last_medal_update_time: i64,
    47 => face_store_id: i64,
    48 => font_effect: i64,
    49 => d_ov_id: String,
    50 => both_flag: i64,
    51 => centi_show_3d_flag: u8,
    52 => intimate_info: Bytes,
    53 => show_nameplate: u8,
    54 => new_lover_diamond_flag: u8,
    55 => ext_sns_frd_data: Bytes,
    56 => mutual_mark_data: Bytes,
});

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct FriendListResponse {
    #[jce(5)]
    pub total_friend_count: i16,
    #[jce(7)]
    pub friend_info_list: Vec<FriendInfo>,
    #[jce(14)]
    pub group_info_list: Vec<FriendListGroupInfo>,
    #[jce(17)]
    pub online_friend_count: i16,
}

/// 好友列表分组信息
#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct FriendListGroupInfo {
    #[jce(0)]
    pub group_id: u8,
    #[jce(1)]
    pub group_name: String,
    #[jce(2)]
    pub friend_count: i32,
    #[jce(3)]
    pub online_friend_count: i32,
    #[jce(4)]
    pub seq_id: u8,
}