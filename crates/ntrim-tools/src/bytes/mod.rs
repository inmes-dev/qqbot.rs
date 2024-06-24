mod builder;
mod reader;

use bitflags::bitflags;
pub use builder::BytePacketBuilder;
pub use reader::BytePacketReader;

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct PacketFlag: u32 {
        const I16Len =  0b00000001;
        const I32Len =  0b00000010;
        const I64Len =  0b00000100;
        const ExtraLen =0b00001000;
        const None =    0b00010000;
    }
}


