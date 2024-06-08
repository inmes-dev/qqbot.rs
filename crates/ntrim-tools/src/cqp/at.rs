use std::fmt::{Display, Formatter};
use crate::cqp::{encode_cq_code_param, SpecialCQCode};

#[derive(Debug, Clone)]
pub struct At {
    pub qq: u64,
    #[cfg(feature = "extend_cqcode")]
    pub content: String,
}

impl Display for At {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if cfg!(feature = "extend_cqcode") {
            write!(f, "[CQ:at,qq={},content={}]", self.qq, encode_cq_code_param(&self.content))
        } else {
            write!(f, "[CQ:at,qq={}]", self.qq)
        }
    }
}

impl SpecialCQCode for At {
    fn get_type(&self) -> String {
        "at".to_string()
    }
}
