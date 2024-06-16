use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Error};
use crate::cqp::{encode_cq_code_param};

#[derive(Debug, Clone, Default)]
pub struct At {
    pub qq: i64,
    #[cfg(feature = "extend_cqcode")]
    pub content: String,
}

impl Display for At {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if cfg!(feature = "extend_cqcode") {
            #[cfg(feature = "extend_cqcode")]
            return write!(f, "[CQ:at,qq={},content={}]", self.qq, encode_cq_code_param(&self.content));
        }
        write!(f, "[CQ:at,qq={}]", self.qq)
    }
}

impl At {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let qq = params.get("qq").ok_or(anyhow!("At 缺少 'qq' 参数"))?.parse::<i64>()?;
        Ok(At {
            qq,
            ..Default::default()
        })
    }
}

