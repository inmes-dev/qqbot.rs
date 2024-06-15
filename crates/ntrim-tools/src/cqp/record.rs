use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Error};
use crate::cqp::encode_cq_code_param;

#[derive(Debug, Default)]
pub struct Record {
    pub file: String,
    pub url: Option<String>,
    pub magic: Option<bool>,
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:record,file={},url={},magic={}]", self.file, encode_cq_code_param(self.url.as_ref().unwrap_or(&"".to_string())), self.magic.unwrap_or(false))
    }
}

impl Record {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let file = params.get("file").ok_or(anyhow!("Record 缺少 'file' 参数"))?;
        let url = params.get("url").map(|s| s.to_string());
        let magic = params.get("magic").map(|s| s.parse::<bool>().unwrap_or(false));
        Ok(Record {
            file: file.to_string(),
            url,
            magic
        })
    }
}