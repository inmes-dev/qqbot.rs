use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct Touch {
    pub id: i32,
}

impl std::fmt::Display for Touch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:touch,id={}]", self.id)
    }
}

impl Touch {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let id = params.get("id").ok_or(anyhow!("Touch 缺少 'id' 参数"))?.parse::<i32>()?;
        Ok(Touch {
            id,
        })
    }
}