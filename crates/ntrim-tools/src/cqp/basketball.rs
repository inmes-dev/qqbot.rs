use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct Basketball {
    pub id: i32,
}

impl std::fmt::Display for Basketball {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:basketball,id={}]", self.id)
    }
}

impl Basketball {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        Ok(Basketball {
            ..Default::default()
        })
    }
}