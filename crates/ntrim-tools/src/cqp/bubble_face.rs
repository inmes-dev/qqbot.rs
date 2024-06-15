use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct BubbleFace {
    pub id: i32,
    pub count: i32,
}

impl std::fmt::Display for BubbleFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:bubble_face,id={},count={}]", self.id, self.count)
    }
}

impl BubbleFace {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let id = params.get("id").ok_or(anyhow!("BubbleFace 缺少 'id' 参数"))?.parse::<i32>()?;
        let count = params.get("count").ok_or(anyhow!("BubbleFace 缺少 'count' 参数"))?.parse::<i32>()?;
        Ok(BubbleFace {
            id,
            count,
        })
    }
}