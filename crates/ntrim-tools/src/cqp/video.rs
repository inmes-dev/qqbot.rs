use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Error};
use crate::cqp::encode_cq_code_param;

#[derive(Debug, Default)]
pub struct Video {
    pub file: String,
    pub url: Option<String>,
}

impl Display for Video {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(url) = &self.url {
            write!(f, "[CQ:video,file={},url={}]", self.file, encode_cq_code_param(url))
        } else {
            write!(f, "[CQ:video,file={}]", self.file)
        }
    }
}

impl Video {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let file = params.get("file").ok_or(anyhow!("Video 缺少 'file' 参数"))?;
        let url = params.get("url").map(|s| s.to_string());
        Ok(Video {
            file: file.to_string(),
            url,
        })
    }
}