use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct Poke {
    pub poke_type: i32,
    pub id: i32,
    pub strength: Option<i32>,
}

impl std::fmt::Display for Poke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(strength) = self.strength {
            write!(f, "[CQ:poke,type={},id={},strength={}]", self.poke_type, self.id, strength)
        } else {
            write!(f, "[CQ:poke,type={},id={}]", self.poke_type, self.id)
        }
    }
}

impl Poke {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let poke_type = params.get("type").ok_or(anyhow!("Poke 缺少 'type' 参数"))?.parse::<i32>()?;
        let id = params.get("id").ok_or(anyhow!("Poke 缺少 'id' 参数"))?.parse::<i32>()?;
        let strength = params.get("strength").map(|s| s.parse::<i32>()).transpose()?;
        Ok(Poke {
            poke_type,
            id,
            strength,
        })
    }
}