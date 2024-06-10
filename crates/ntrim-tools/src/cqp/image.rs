use std::fmt::{Display, Formatter};
use crate::cqp::{encode_cq_code_param, SpecialCQCode};

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

impl SpecialCQCode for Image {
    fn get_type(&self) -> String {
        "image".to_string()
    }
}