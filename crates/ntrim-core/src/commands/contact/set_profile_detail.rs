use bytes::{BufMut, Bytes, BytesMut};
use ntrim_macros::command;
use prost::Message;
use crate::oidb_request;

struct SetProfileDetailCodec;

#[command("OidbSvc.0x4ff_9_IMCore", "set_profile_detail", Service, Protobuf)]
impl SetProfileDetailCodec {
    async fn generate(bot: &Arc<Bot>, profile_card: Vec<(u16, Bytes)>) -> Option<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put_u32(bot.unique_id as u32);
        buf.put_u8(0);
        buf.put_u16(profile_card.len() as u16);
        for (field, value) in profile_card {
            buf.put_u16(field);
            buf.put_u16(value.len() as u16);
            buf.put(value);
        }
        oidb_request!(0x4ff, 9, buf.to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<()> {
        None
    }
}