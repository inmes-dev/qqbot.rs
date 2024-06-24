use std::sync::Arc;
use anyhow::anyhow;
use bytes::{Buf, BufMut};
use log::info;
use prost::Message;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::Method;
use ntrim_tools::bytes::{BytePacketBuilder, PacketFlag};
use crate::bot::Bot;
use crate::pb::bdh::{DataHighwayHead, DataHighwaySegHead, LoginSigHead, ReqDataHighwayHead, RespDataHighway};

pub(super) fn upload_block_no_resp(
    bot: Arc<Bot>,
    client: &Client,
    addr: &String,
    head: Vec<u8>,
    block: &[u8]
) -> anyhow::Result<()> {
    let bot_id = bot.unique_id;
    let mut buffer = Vec::new();
    buffer.put_u8(0x28);
    buffer.put_u32(head.len() as u32);
    buffer.put_u32(block.len() as u32);
    buffer.put_bytes_with_flags(&head, PacketFlag::None);
    buffer.put_bytes_with_flags(block, PacketFlag::None);
    buffer.put_u8(0x29);
    let url = format!("http://{}/cgi-bin/httpconn", addr);
    //info!("[debug_bdh] BDH Request Url: {:?}", url);
    //info!("[debug_bdh] BDH Post Len: {:?}", buffer.len());
    let response = client.request(Method::POST, url)
        .query(&[
            ("htcmd", "0x6FF0087".to_string()),
            ("uin", bot_id.to_string())
        ])
        .timeout(std::time::Duration::from_secs(60))
        .body(buffer)
        .send()?;
    //info!("[debug_bdh] Upload Block Response: {:?}", response.status());
    if response.status().is_success() {
        let mut buf = response.bytes()?;
        buf.advance(1);
        let head_len = buf.get_u32();
        buf.advance(4);
        let response = buf.split_to(head_len as usize);
        let response = RespDataHighway::decode(response)?;
        if response.err_code != 0 {
            return Err(anyhow!("Upload Block Response Error: {:?}", response.err_code));
        } else {
            info!("Upload Block Response: {:?}", response)
        }
    } else {
        return Err(anyhow!("Request failed: {:?}", response.status()));
    }
    return Ok(());
}