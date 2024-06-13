
use jcers::{JceGet, JcePut};
use bytes::Bytes;

// friendlist.GetMultiTroopInfoReq
jce_struct!(TroopMultiInfoRequest {
    0 => uin: i64,
    1 => group_list: Vec<i64>,
    2 => rich_info: u8,
});

jce_struct!(TroopMultiinfoResponse {
    0 => uin: i64,
    1 => result: i32,
    2 => err_code: i16,
    3 => group_info: Vec<TroopInfoV2>,
    4 => group_class_xml_path: String,
});

jce_struct!(TroopInfoV2 {
    0 => group_uin: i64,
    1 => group_code: i64,
    2 => group_name: String,
    3 => group_memo: String,
    4 => group_owner: i64,
    5 => group_class_ext: i64,
    6 => group_face: i32,
    7 => group_finger_memo: String,
    8 => group_option: u8,
    9 => member_num: i32,
    10 => group_flag_ext: i64,
});