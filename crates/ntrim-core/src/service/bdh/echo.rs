use std::sync::Arc;
use anyhow::anyhow;
use bytes::{Buf, BufMut, BytesMut};
use log::info;
use prost::Message;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use ntrim_tools::bytes::{BytePacketBuilder, PacketFlag};
use crate::bot::Bot;
use crate::pb::bdh::{ * };
use crate::service::bdh::build_req_head;

pub(super) fn echo(
    bot: Arc<Bot>,
    client: &Client,
    addr: &String,
    seq: u32
) -> anyhow::Result<()> {
    let bot_id = bot.unique_id;
    let head = build_req_head(
        &bot, "PicUp.Echo".to_string(), 91, seq, 0, 0, 0, 0,
        None, None, None, None, None
    ).encode_to_vec();
    let mut buffer = Vec::new();
    buffer.put_u8(0x28);
    buffer.put_u32(head.len() as u32);
    buffer.put_u32(0);
    buffer.put_bytes_with_flags(&head, PacketFlag::None);
    buffer.put_u8(0x29);
    let url = format!("http://{}/cgi-bin/httpconn?htcmd=0x6ff0082&uin={}", addr, bot_id);
    let response = client
        .post(url)
        .body(buffer)
        .send()?;
    if response.status().is_success() {
        let mut buf = response.bytes()?;
        buf.advance(1);
        let head_len = buf.get_u32();
        buf.advance(4);
        let response = buf.split_to(head_len as usize);
        let response = RespDataHighway::decode(response)?;
        if response.err_code != 0 {
            return Err(anyhow!("Echo Response Error: {:?}", response.err_code));
        }
    } else {
        return Err(anyhow!("Request failed: {:?}", response.status()));
    }
    return Ok(());
}