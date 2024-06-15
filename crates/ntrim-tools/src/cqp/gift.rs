use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct Gift {
    pub qq: i32,
    pub id: i32,
}

impl std::fmt::Display for Gift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:gift,qq={},id={}]", self.qq, self.id)
    }
}

impl Gift {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let qq = params.get("qq").ok_or(anyhow!("Gift 缺少 'qq' 参数"))?.parse::<i32>()?;
        let id = params.get("id").ok_or(anyhow!("Gift 缺少 'id' 参数"))?.parse::<i32>()?;
        Ok(Gift {
            qq,
            id,
        })
    }
}