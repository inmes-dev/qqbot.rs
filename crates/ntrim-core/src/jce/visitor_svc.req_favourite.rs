use jcers::{JceGet, JcePut};
use bytes::Bytes;

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct QQServiceReqHead {
    #[jce(0)]
    pub uin: i64,
    #[jce(1)]
    pub sh_version: i16,
    #[jce(2)]
    pub seq: i32,
    #[jce(3)]
    pub req_type: u8,
    #[jce(4)]
    pub triggered: u8,
    #[jce(5)]
    pub cookies: Bytes,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct ReqFavorite {
    #[jce(0)]
    pub header: QQServiceReqHead,
    #[jce(1)]
    pub mid: i64,
    #[jce(2)]
    pub op_type: i32,
    #[jce(3)]
    pub source: i32,
    #[jce(4)]
    pub count: i32,
    #[jce(5)]
    pub t: i32,
}