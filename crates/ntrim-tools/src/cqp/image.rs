use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Error};
use crate::cqp::{encode_cq_code_param};

#[derive(Debug, Clone, Default)]
pub struct Image {
    pub file: String,
    pub url: String,
    pub r#type: String,
    pub sub_type: u32
}

impl Image {
    pub fn new(file: String, url: String, original: bool) -> Self {
        Self {
            file,
            url,
            r#type: if original {
                "original".to_string()
            } else {
                "show".to_string()
            },
            sub_type: 0
        }
    }

    pub fn with_sub_type(file: String, url: String, sub_type: u32) -> Self {
        Self {
            file,
            url,
            r#type: "show".to_string(),
            sub_type
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:image,file={},url={},type={},subType={}]", self.file, encode_cq_code_param(&self.url), self.r#type, self.sub_type)
    }
}

impl Image {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let file = params.get("file").ok_or(anyhow!("Image 缺少 'file' 参数"))?;
        let url = params.get("url").ok_or(anyhow!("Image 缺少 'url' 参数"))?;
        let r#type = params.get("type").ok_or(anyhow!("Image 缺少 'type' 参数"))?;
        let sub_type = params.get("subType").map(|s| s.parse::<u32>().unwrap_or(0)).unwrap_or(0);
        Ok(Image {
            file: file.to_string(),
            url: url.to_string(),
            r#type: r#type.to_string(),
            sub_type
        })
    }
}
