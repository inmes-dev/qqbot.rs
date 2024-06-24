use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;
use log::info;
use once_cell::sync::Lazy;
use prost::Message as _;
use ntrim_macros::command;
use crate::{oidb_request, oidb_response};
use crate::commands::richmedia::{ * };
use crate::pb::msg::ExtBizInfo;
use crate::pb::trpc::rich_media_ntv2::{ * };

struct RequestUploadResourceCodec;

#[command("OidbSvcTrpcTcp.0x11c4_100", "_request_upload_resource", Protobuf, Service)]
impl RequestUploadResourceCodec {
    async fn generate(
        bot: &Arc<Bot>,
        c2c_user_info: Option<C2cUserInfo>,
        group_user_info: Option<GroupUserInfo>,
        channel_user_info: Option<ChannelUserInfo>,
        file_info: FileInfo,
        ext_biz_info: ExtBizInfo
    ) -> Option<Vec<u8>> {
       let scene_type = if c2c_user_info.is_some() {
            SCENE_C2C
        } else if group_user_info.is_some() {
            SCENE_GROUP
        } else if channel_user_info.is_some() {
            SCENE_CHANNEL
        } else {
            SCENE_UNKNOWN
        };
        let data = oidb_request!(0x11c4, 100, NtV2RichMediaReq {
            head: MultiMediaReqHead {
                head: CommonHead {
                    req_id: next_rich_media_seq(bot.unique_id),
                    cmd: 100,
                    msg: None,
                },
                scene: SceneInfo {
                    request_type: 2,
                    business_type: 1,
                    app_type: None,
                    scene_type: Some(scene_type),
                    c2c: c2c_user_info,
                    grp: group_user_info,
                    channel: channel_user_info,
                    byte_arr: None,
                },
                client_meta: ClientMeta {
                    agent_type: 2,
                },
            },
            upload: Some(UploadReq {
                no_need_compat_msg: Some(true),
                client_seq: Some(28321), // 28321
                ext_biz_info: Some(ext_biz_info),
                compat_q_msg_scene_type: Some(scene_type),
                client_random_id: Some(1009114165),
                srv_send_msg: Some(false),
                try_fast_upload_completed: Some(true),
                upload_info: vec![UploadInfo {
                    sub_file_type: 0,
                    file_info
                }]
            }),
            download: None,
        }.encode_to_vec());
        //if let Some(data) = &data {
        //    info!("Generated request upload resource: {:?}", hex::encode(data));
        //}
        data
        //Some(hex::decode("08 C4 23 10 64 22 9F 02 0A 1E 0A 04 08 07 10 64 12 12 A8 06 02 B0 06 01 C0 0C 02 D2 0C 06 08 BD AC FA B3 02 1A 02 08 02 12 FC 01 0A 90 01 0A 8B 01 08 B3 E7 07 12 20 39 62 36 32 38 61 64 62 39 33 35 62 64 31 32 61 62 34 30 31 39 63 64 31 64 34 66 64 33 61 64 34 1A 28 65 66 31 34 35 32 39 63 35 39 37 64 34 38 30 63 61 61 62 64 37 31 63 62 33 66 32 30 39 39 39 37 63 62 34 35 63 61 35 38 22 24 39 42 36 32 38 41 44 42 39 33 35 42 44 31 32 41 42 34 30 31 39 43 44 31 44 34 46 44 33 41 44 34 2E 6A 70 67 2A 09 08 01 10 E8 07 18 00 20 00 30 C0 0C 38 80 14 40 00 48 01 10 00 10 01 18 00 20 96 BC 9B E3 07 28 02 32 56 0A 4A 08 00 12 00 62 44 08 00 18 00 20 00 4A 00 50 00 62 00 92 01 00 9A 01 00 AA 01 0C 08 00 12 00 18 00 20 00 28 00 3A 00 FA 01 20 39 42 36 32 38 41 44 42 39 33 35 42 44 31 32 41 42 34 30 31 39 43 44 31 44 34 46 44 33 41 44 34 12 02 1A 00 1A 04 5A 00 62 00 38 CB 21 40 01 60 01".replace(" ", "")).unwrap().to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<UploadRsp> {
        let response = oidb_response!(0x11c4, 100, data.as_slice())?;
        match NtV2RichMediaRsp::decode(response.as_slice()) {
            Ok(v) => {
                if (v.head.ret_code.is_none() || v.head.ret_code == Some(0)) && v.upload.is_some() {
                    Some(v.upload?)
                } else {
                    error!("Failed to request upload resource, code: {:?}, msg: {}, resp: {}", v.head.ret_code, v.head.msg, hex::encode(&data));
                    None
                }
            },
            Err(e) => {
                error!("Failed to decode NtV2RichMediaRsp(100): {:?}, data: {}", e, hex::encode(&response));
                None
            }
        }
    }
}
