use std::collections::HashMap;
use anyhow::Error;

#[derive(Debug, Default)]
pub struct NewRPS {
    pub id: i32,
}

impl std::fmt::Display for NewRPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:new_rps,id={}]", self.id)
    }
}

impl NewRPS {
    pub(crate) fn from(_params: &HashMap<String, String>) -> Result<Self, Error> {
        Ok(NewRPS {
            ..Default::default()
        })
    }
}