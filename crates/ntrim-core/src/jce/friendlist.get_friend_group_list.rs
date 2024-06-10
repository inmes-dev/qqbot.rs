use jcers::{JceGet, JcePut};
use bytes::Bytes;

jce_struct!(FriendListRequest {
    0 => reqtype: i32,
    1 => if_reflush: u8,
    2 => uin: i64,
    3 => start_index: i16,
    4 => friend_count: i16,
    5 => group_id: u8,
    6 => if_get_group_info: u8,
    7 => group_start_index: u8,
    8 => group_count: u8,
    9 => if_get_msf_group: u8,
    10 => if_show_term_type: u8,
    11 => version: i64,
    12 => uin_list: Vec<i64>,
    13 => app_type: i32,
    14 => if_get_dov_id: u8,
    15 => if_get_both_flag: u8,
    16 => d50: Bytes,
    17 => d6b: Bytes,
    18 => sns_type_list: Vec<i64>,
});