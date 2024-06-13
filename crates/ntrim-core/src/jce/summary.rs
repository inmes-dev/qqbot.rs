use crate::jce_struct;
use jcers::{JceGet, JcePut};
use bytes::Bytes;

jce_struct!(SummaryCardReq {
    0 => uin: i64,
    1 => come_from: i32,
    2 => qzone_feed_timestamp: i64,
    3 => is_friend: u8,
    4 => group_code: i64,
    5 => group_uin: i64,
    6 => seed: Bytes,
    7 => search_name: String,
    8 => get_control: i64,
    9 => add_friend_source: i32,
    10 => secure_sig: Bytes,
    12 => req_tmp_info: Bytes,
    14 => req_services: Vec<Bytes>, // todo
    15 => tiny_id: i64,
    16 => like_source: i64,
    18 => req_medal_wall_info: u8,
    19 => req_0x5eb_field_id: Vec<i64>,
    20 => req_nearby_god_info: u8,
    22 => req_extend_card: u8,
    24 => u24: u8,
});

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct RespSummaryCardHead {
    #[jce(0)]
    pub sex: i32,
    #[jce(1)]
    pub age: i32,
    #[jce(2)]
    pub err_str: String,
    #[jce(3)]
    pub cookie: Bytes,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct RespSummaryCard {
    #[jce(1)]
    pub sex: u8,
    #[jce(2)]
    pub age: u8,
    #[jce(3)]
    pub nickname: String,
    #[jce(5)]
    pub level: i32,
    #[jce(7)]
    pub city: String,
    #[jce(8)]
    pub sign: String,
    #[jce(11)]
    pub mobile: String,
    #[jce(23)]
    pub uin: i64,
    #[jce(36)]
    pub login_days: i64,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct MCardSvc {
    #[jce(0)]
    pub ver: u8,
    #[jce(1)]
    pub uin: i64,
    #[jce(2)]
    pub target: i64,
    #[jce(3)]
    pub qq_ver: String,
    #[jce(4)]
    pub come_from: i32,
    #[jce(5)]
    pub plat: i32
}