use std::collections::HashMap;
use anyhow::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use log::info;
use ntrim_macros::command;
use crate::{oidb_request, oidb_response};
use prost::{DecodeError, Message};
use serde::Serialize;
use time::macros::date;

pub struct GetProfileDetailCodec;

const FIELD_NICK: u16 =         20002;
const FIELD_SEX: u16 =          20009;
const FIELD_EMAIL: u16 =        20011;
const FIELD_PERSONAL_NOTE: u16 =20019;
const FIELD_COLLEGE: u16 =      20021;
const FIELD_GET_BIRTHDAY: u16 = 20031;
const FIELD_QZONE_LOCATION: u16=20032;
const FIELD_AGE: u16 =          20037;
const FIELD_QZONE_LOCATION_DISTRICT: u16 = 20041;
const FIELD_QZONE_HOMETOWN_DISTRICT: u16 = 20043;
const FIELD_QZONE_HOMETOWN: u16 =          24002;
const FIELD_COMPANY: u16 =      24008;
const FIELD_SET_BIRTHDAY: u16 = 26003;
const FIELD_PROFESSION: u16 =   27037;
const FIELD_PERSONALITY_LABEL_SWITCH: u16 = 42128;
const FIELD_ONLINE_STATUS: u16 =42257;
const FIELD_STICKY_NOTE_OFFLINE: u16 = 45168;

#[derive(Debug, Serialize)]
pub struct QQProfile {
    pub nick_name: String,
    pub sex: i16,
}

#[command("OidbSvc.0x480_9_IMCore", "get_profile_detail", Service, Protobuf)]
impl GetProfileDetailCodec {
    async fn generate(bot: &Arc<Bot>, user_id: i64) -> Option<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put_u32(user_id as u32);
        buf.put_u8(0);

        buf.put_u16(13);
        buf.put_u16(FIELD_SEX);
        buf.put_u16(FIELD_PROFESSION);
        buf.put_u16(FIELD_GET_BIRTHDAY);
        buf.put_u16(FIELD_COMPANY);
        buf.put_u16(FIELD_QZONE_HOMETOWN);
        buf.put_u16(FIELD_QZONE_HOMETOWN_DISTRICT);
        buf.put_u16(FIELD_QZONE_LOCATION);
        buf.put_u16(FIELD_QZONE_LOCATION_DISTRICT);
        buf.put_u16(FIELD_EMAIL);
        buf.put_u16(FIELD_PERSONAL_NOTE);
        buf.put_u16(FIELD_COLLEGE);
        buf.put_u16(FIELD_AGE);
        buf.put_u16(FIELD_NICK);

        oidb_request!(1152, 9, buf.to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<QQProfile> {
        let data = oidb_response!(1152, 9, data.as_slice())?;
        let mut data = Bytes::from(data);
        let _ = data.get_u32();
        data.advance(1);
        let size = data.get_u16();
        let mut sex = -1i16;
        let mut nick: String = "".to_string();
        for _ in 0..size {
            let id = data.get_u16();
            if id == 0 {
                break;
            }
            let size = data.get_u16();
            match id {
                FIELD_SEX => {
                    sex = data.get_i8() as i16;
                },
                FIELD_NICK => {
                    let tmp = data.copy_to_bytes(size as usize);
                    nick = String::from_utf8(tmp.to_vec()).unwrap();
                },
                _ => {
                    data.advance(size as usize)
                }
            }
        }
        Some(QQProfile {
            nick_name: nick,
            sex
        })
    }
}