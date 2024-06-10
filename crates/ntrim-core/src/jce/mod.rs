use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;

use bytes::{BufMut, Bytes, BytesMut};
use jcers::{JceGet, JcePut};
use once_cell::sync::Lazy;

#[macro_export]
macro_rules! jce_struct {
    ($struct_name: ident {$($tag: expr => $field: ident: $field_t: ty,)*}) => {
        #[derive(Debug, Clone, PartialEq, Eq, JceGet, JcePut, Default)]
        pub struct $struct_name {
            $(#[jce($tag)]
            pub $field: $field_t),*
        }
    };
}

pub fn next_request_id(uin: i64) -> i32 {
    static MAP_REQUEST_ID: Lazy<Mutex<HashMap<i64, AtomicU32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
    let mut request_id = MAP_REQUEST_ID.lock().unwrap();
    let seq = request_id.entry(uin).or_insert(AtomicU32::new(17050));
    if seq.load(std::sync::atomic::Ordering::Relaxed) >= 0x8000 {
        seq.store(17050, std::sync::atomic::Ordering::Relaxed);
    }
    seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed) as i32
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct RequestPacket {
    #[jce(1)]
    pub i_version: i16,
    #[jce(2)]
    pub c_packet_type: u8,
    #[jce(3)]
    pub i_message_type: i32,
    #[jce(4)]
    pub i_request_id: i32,
    #[jce(5)]
    pub s_servant_name: String,
    #[jce(6)]
    pub s_func_name: String,
    #[jce(7)]
    pub s_buffer: Bytes,
    #[jce(8)]
    pub i_timeout: i32,
    #[jce(9)]
    pub context: HashMap<String, String>,
    #[jce(10)]
    pub status: HashMap<String, String>,
}

jce_struct!(RequestDataVersion3 {
    0 => map: HashMap<String,Bytes>,
});

jce_struct!(RequestDataVersion2 {
    0 => map: HashMap<String,HashMap<String,Bytes>>,
});

pub fn pack_uni_request_data(data: &[u8]) -> Bytes {
    let mut r = BytesMut::new();
    r.put_u8(0x0A);
    r.put_slice(data);
    r.put_u8(0x0B);
    Bytes::from(r)
}

pub mod onlinepush {
    pub mod reqpushmsg {
        include!("onlinepush.reqpushmsg.rs");
    }
}

pub mod friendlist {
    pub mod get_troop_list {
        include!("friendlist.get_troop_list.rs");
    }

    pub mod get_troop_member_list {
        include!("friendlist.get_troop_member_list.rs");
    }

    pub mod get_friend_group_list {
        include!("friendlist.get_friend_group_list.rs");
    }
}