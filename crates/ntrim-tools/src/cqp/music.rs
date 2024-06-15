use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct Music {
    pub music_type: String,
    pub id: i32,
}

impl std::fmt::Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:music,type={},id={}]", self.music_type, self.id)
    }
}

impl Music {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let music_type = params.get("type").ok_or(anyhow!("Music 缺少 'type' 参数"))?;
        let id = params.get("id").ok_or(anyhow!("Music 缺少 'id' 参数"))?.parse::<i32>()?;
        Ok(Music {
            music_type: music_type.to_string(),
            id,
        })
    }
}