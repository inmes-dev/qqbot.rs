mod echo;
mod upload_block_no_resp;

use std::sync::Arc;
use anyhow::anyhow;
use bytes::BufMut;
use log::{info, warn};
use nom::Slice;
use prost::Message;
use reqwest::blocking::Client;
use ntrim_tools::bytes::{BytePacketBuilder};
use ntrim_tools::tokiort::global_tokio_runtime;
use crate::bot::Bot;
use crate::pb::bdh::{DataHighwayHead, DataHighwaySegHead, LoginSigHead, ReqDataHighwayHead};
use crate::service::bdh::echo::echo;
use crate::service::bdh::upload_block_no_resp::upload_block_no_resp;
use crate::session::ticket::{SigType};

pub(crate) async fn upload_resource_no_resp(
    bot: Arc<Bot>,
    cmd: String,
    cmd_id: u32,
    chunk_size: usize,
    vec_data: Vec<u8>,
    file_md5: Vec<u8>,
    ext: Vec<u8>
) -> anyhow::Result<()> {
    let ticket = Bot::get_upload_key(&bot).await?;
    let ip = ticket.1[0].0;
    let port = ticket.1[0].1;
    let addr = format!("{}.{}.{}.{}:{}", ip & 0xff, (ip >> 8) & 0xff, (ip >> 16) & 0xff, ip >> 24, port);
    //info!("[debug_bdh] BDH addr: {}", addr);
    //info!("[debug_bdh] BDH ticket: {}", hex::encode(&ticket.0));

    let session = bot.client.session.read().await;
    let sub_app_id = session.protocol.sub_app_id;
    let tgt = session.tickets.get(&SigType::A2)
        .unwrap().sig.clone()
        .unwrap();
    drop(session); // 释放读锁
    let login_sig_head = LoginSigHead {
        sig_type: 8,
        sig: tgt,
    };

    return global_tokio_runtime().spawn_blocking(move || {
        let mut client = Client::builder()
            .danger_accept_invalid_certs(true)
            .user_agent("Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2)");
        let client = client.build()?;

        //info!("[debug_bdh] Echoing...");
        if let Err(e) = echo(bot.clone(), &client, &addr, 0) {
            return Err(anyhow!("Failed to echo: {}", e));
        }
        //info!("[debug_bdh] Echoed");

        let data = vec_data.as_slice();
        let len = data.len();
        let mut seq = 1u32;
        for i in (0..len).step_by(chunk_size) {
            let min = std::cmp::min(i + chunk_size, len);
            let chunk = data.slice(i..min);
            let chunk_md5 = md5::compute(chunk).to_vec();

            let head = build_req_head(
                &bot,
                cmd.clone(),
                cmd_id,
                seq,
                data.len() as u64,
                i as u32,
                chunk.len() as u32,
                sub_app_id,
                Some(ticket.0.clone()),
                Some(chunk_md5),
                Some(file_md5.clone()),
                Some(ext.clone()),
                Some(login_sig_head.clone()),
            ).encode_to_vec();

            //info!("Head buf: {:?}", hex::encode(&head));

            //info!("[debug_bdh] Uploading block {}/{}", seq, len / chunk_size + 1);
            if let Err(e) = upload_block_no_resp(bot.clone(), &client, &addr, head, chunk) {
                return Err(anyhow!("Failed to upload block: {}", e));
            }
            //info!("Uploaded block {}/{}", seq, len / chunk_size + 1);

            seq += 1;
        }
        Ok(())
    }).await?;
}

fn build_req_head(
    bot: &Arc<Bot>,
    cmd: String,
    cmd_id: u32,
    seq: u32,
    file_size: u64,
    offset: u32,
    block_size: u32,
    sub_app_id: u32,
    ticket: Option<Vec<u8>>,
    block_md5: Option<Vec<u8>>,
    file_md5: Option<Vec<u8>>,
    ext: Option<Vec<u8>>,
    login_sig_head: Option<LoginSigHead>,
) -> ReqDataHighwayHead {
    ReqDataHighwayHead {
        head: DataHighwayHead {
            ver: 1,
            uin: bot.unique_id.to_string(),
            cmd,
            seq,
            retry_tomes: 0,
            sub_app_id,
            data_flag: 16,
            cmd_id,
        },
        seg_head: DataHighwaySegHead {
            service_id: 0,
            file_size,
            offset,
            block_size,
            service_ticket: ticket,
            block_md5,
            file_md5,
            cache_ip: 0,
            cache_port: 0,
        },
        ext,
        flag: Some(0),
        login_sig_head,
        ..Default::default()
    }
}

#[test]
fn int32_to_ipv4() {
    let ip = 1697048952u32;
    let ip_str = format!("{}.{}.{}.{}", ip >> 24, (ip >> 16) & 0xff, (ip >> 8) & 0xff, ip & 0xff); // 翻转
    let re_ip_str = format!("{}.{}.{}.{}", ip & 0xff, (ip >> 8) & 0xff, (ip >> 16) & 0xff, ip >> 24); // 正常
    println!("ip_str: {}, {}", ip_str, re_ip_str);
}