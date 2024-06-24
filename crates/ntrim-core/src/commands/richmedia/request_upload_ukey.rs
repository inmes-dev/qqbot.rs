use bytes::{Buf, Bytes};
use log::info;
use prost::Message as _;
use ntrim_macros::command;
use ntrim_tools::protobuf;
use pb::trpc::http_conn::HttpConn0xff501Response;
use crate::{oidb_request, oidb_response, pb};
use crate::pb::trpc::rich_media_ntv2::{ * };

struct RequestUploadUKey;

#[command("HttpConn.0x6ff_501", "_request_upload_ukey", Protobuf, Service)]
impl RequestUploadUKey {
    async fn generate(bot: &Arc<Bot>) -> Option<Vec<u8>> {
        let mut buf = Vec::new();
        let mut buf1281 = Vec::new();
        protobuf!(buf1281, {
            1 => int64 => bot.unique_id,
            2 => int32 => 0, // idc id
            3 => int32 => 16, // app_id
            4 => int32 => 1, // login sig type
            6 => int32 => 3, // request flag (4 for redpacket)
        });
        // 5 nearby
        // 10 21 hw
        // 1 big data
        prost::encoding::int32::encode_repeated(7, &[1, 10, 21], &mut buf1281); // rpt_uint32_service_types
        prost::encoding::int32::encode(10, &9, &mut buf1281); // plat
        protobuf!(buf, {
            1281 => bytes => buf1281
        });
        Some(buf)
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<(Vec<u8>, Vec<(u32, u32)>)> {
        //info!("RequestUploadUKey: {:?}", hex::encode(&data));
        let mut rsp = HttpConn0xff501Response::decode(Bytes::from(data))
            .map_err(|e| {
                warn!("Failed to decode HttpConn0xff501Response: {:?}", e);
            })
            .ok()?;
        let ukey = rsp.body.session_ticket;
        let mut addrs = Vec::new();
        for server in rsp.body.servers {
            for addr in server.server_addr {
                addrs.push((addr.ip, addr.port));
            }
        }
        Some((ukey, addrs))
    }
}