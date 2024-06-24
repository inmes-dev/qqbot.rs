use std::path::Path;
use std::result;
use std::sync::Arc;
use anyhow::{anyhow, Error};
use image::{DynamicImage, GenericImageView};
use log::{debug, info};
use prost::Message;
use sha1::{Digest, Sha1};
use ntrim_tools::tokiort::global_tokio_runtime;
use crate::await_response;
use crate::bot::Bot;
use crate::pb::bdh::{Ip, Network, NtHighwayHash, Opt, RichMediaExt};
use crate::pb::msg::{ExtBizInfo, MsgInfo, PicExtBizInfo, PttExtBizInfo, VideoExtBizInfo};
use crate::pb::msg::pic_ext_biz_info::PicExtReserveTroop;
use crate::pb::trpc::rich_media_ntv2::{*};
use crate::service::bdh;

impl Bot {
    pub async fn upload_group_pic(
        self: &Arc<Bot>,
        group_id: i64,
        pic_path: String,
        original: bool
    ) -> anyhow::Result<MsgInfo> {
        let pic_bytes = std::fs::read(&pic_path)
            .map_err(|e| anyhow!("read pic file err: {}", e))?;
        let mut hasher = Sha1::new();
        hasher.update(&pic_bytes);
        let sha1 = hasher.finalize().to_vec();
        let sha1_str = hex::encode(&sha1).to_ascii_lowercase();
        let md5 = md5::compute(&pic_bytes).to_vec();
        let md5_str = hex::encode(&md5).to_ascii_uppercase();
        let (width, height) = {
            let img = image::open(&pic_path);
            if img.is_err() {
                (256, 256)
            } else {
                let img = img.unwrap();
                img.dimensions()
            }
        };
        let file_name = md5_str.clone() + option_env!("_PIC_NAME").unwrap_or(".jpg");
        //info!("file Name: {}", file_name);
        let file_info = FileInfo {
            file_size: Some(pic_bytes.len() as u64),
            md5: Some(md5_str.to_ascii_lowercase()),
            sha1: Some(sha1_str),
            name: Some(file_name),
            file_type: Some(FileType {
                file_type: Some(1),
                pic_format: Some(1000),
                voice_format: Some(0),
                video_format: Some(0),
            }),
            width: Some(width),
            height: Some(height),
            time: Some(0),
            original: Some(if original { 1 } else { 0 }),
        };
        let ext_biz_info = ExtBizInfo {
            pic: Some(PicExtBizInfo {
                biz_type: Some(0),
                text_summary: Some("".to_string()),
                bytes_pb_reserve_troop: Some(PicExtReserveTroop {
                    md5: md5_str.clone(),
                    ..Default::default()
                }.encode_to_vec()),
                ..Default::default()
            }),
            video: Some(VideoExtBizInfo {
                bytes_pb_reserve: Some(vec![]),
                ..Default::default()
            }),
            ptt: Some(PttExtBizInfo {
                bytes_pb_reserve: Some(vec![]),
                bytes_reserve: Some(vec![]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let upload = await_response!(tokio::time::Duration::from_secs(5), async {
            let rx = Bot::_request_upload_resource(self, None, Some(GroupUserInfo {
                uin: group_id as u32
            }), None, file_info, ext_biz_info).await;
            if let Some(rx) = rx {
                rx.await.map_err(|e| Error::new(e))
            } else {
                Err(Error::msg("Tcp connection exception"))
            }
        }, |value: Option<UploadRsp>| {
            if value.is_some() {
                Ok(value.unwrap())
            } else {
                Err(anyhow!("Failed to upload group pic: no upload info"))
            }
        }, |err| {
            Err(err)
        })?;

        let msg_info = upload.msg_info
            .ok_or(anyhow!("Failed to upload group pic: no msg_info"))?;

        if upload.ukey.is_none() {
            //debug!("Pic is exist! {:?}", msg_info);
            return Ok(msg_info)
        }

        //println!("Upload: {:?}", upload);

        let ukey = upload.ukey
            .ok_or(anyhow!("Failed to upload group pic: no ukey"))?;
        let mut msg_info_body = msg_info.msg_info_body.first()
            .ok_or(anyhow!("Failed to upload group pic: no msg_info_body"))?
            .clone();
        let mut index = &mut msg_info_body.index;
        index.sub_type = Some(0);
        index.file_info.duration = Some(0);
        //let hash_sum = msg_info_body.hash_sum.clone()
        //    .ok_or(anyhow!("Failed to upload group pic: no hash_sum"))?;

        let ips = upload.ipv4.iter().map(|item| {
            let ip = item.in_ip.unwrap_or(0);
            let in_port = item.in_port.unwrap_or(0);
            let in_ip = format!("{}.{}.{}.{}", ip & 0xff, (ip >> 8) & 0xff, (ip >> 16) & 0xff, ip >> 24);
            crate::pb::bdh::Ipv4 {
                ip: Some(Ip {
                    enable: 1,
                    ip: in_ip
                }),
                port: in_port as u32
            }
        }).collect::<Vec<_>>();

        //info!("PicUKEY: {}", ukey);
        //info!("PicUUID: {}", index.file_uuid);
        //info!("PicUUID: {:?}", msg_info_body.picture);

        let max_chunk_size = std::env::var("BDH_CHUNK_SIZE")
            .map_or(1024 * 500, |size| size.parse::<usize>().unwrap());
        let ext = RichMediaExt {
            file_uuid: index.file_uuid.clone(),
            up_key: ukey,
            original: if original { 1 } else { 0 },
            opt: Some(Opt {
                switch_1: 1,
                switch_4: 1
            }),
            network: Some(Network {
                addrs_v4: ips
            }),
            msg_info_body: vec![msg_info_body],
            block_size: 1048576,
            nt_highway_hash: Some(NtHighwayHash {
                sha1: vec![sha1]
            }),
            ..Default::default()
        }.encode_to_vec();

        bdh::upload_resource_no_resp(
            self.clone(),
            "PicUp.DataUp".to_string(),
            1004,
            max_chunk_size,
            pic_bytes,
            md5,
            ext
        ).await?;

        Ok(msg_info)
    }

}

#[test]
fn test_parse_pic() {
    let pic_path = Path::new("DA9487B329E63AE4B19DE22EF9CAB892.jpg");
    let pic_bytes = std::fs::read(&pic_path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&pic_bytes);
    let sha1 = hex::encode(hasher.finalize()).to_ascii_lowercase();
    let md5 = hex::encode(md5::compute(&pic_bytes).as_slice()).to_ascii_lowercase();
    let file_name = md5.to_uppercase() + option_env!("_PIC_NAME").unwrap_or(".jpg");
    let (width, height) = {
        let img = image::open(&pic_path);
        if img.is_err() {
            (256, 256)
        } else {
            img.unwrap().dimensions()
        }
    };
    println!("sha1: {}, md5: {}, file_name: {}, width: {}, height: {}", sha1, md5, file_name, width, height)
}