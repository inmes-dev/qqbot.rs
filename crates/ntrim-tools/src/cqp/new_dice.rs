use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct NewDice {
    pub id: i32,
}

impl std::fmt::Display for NewDice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:new_dice,id={}]", self.id)
    }
}

impl NewDice {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        Ok(NewDice {
            ..Default::default()
        })
    }
}