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
        let binding = "".to_string();
        let file = params.get("file").unwrap_or(&binding);
        let url = params.get("url").unwrap_or(&binding);
        let r#type = params.get("type").unwrap_or(&binding);
        let sub_type = params.get("subType").map(|s| s.parse::<u32>().unwrap_or(0)).unwrap_or(0);
        if file.is_empty() && url.is_empty() {
            return Err(anyhow!("file and url can't be empty"))
        }
        Ok(Image {
            file: file.to_string(),
            url: url.to_string(),
            r#type: r#type.to_string(),
            sub_type
        })
    }
}
