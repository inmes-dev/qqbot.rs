pub mod parser;
pub mod at;
pub mod face;
pub mod image;

pub use at::At;
pub use face::Face;
pub use image::Image;
pub use parser::parse_cq;

use std::fmt::Display;
use prost::Message;

pub trait SpecialCQCode: Display + Send + Sync {
    fn get_type(&self) -> String where Self: Sized;
}

pub enum CQCode {
    Special(Box<dyn SpecialCQCode>),
    Text(String),
}

impl Display for CQCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CQCode::Special(cq) => write!(f, "{}", cq),
            CQCode::Text(text) => write!(f, "{}", text),
        }
    }
}

fn encode_cq_code_param(cq: &str) -> String {
    cq.replace("&", "&amp;")
        .replace("[", "&#91;")
        .replace("]", "&#93;")
        .replace(",", "&#44;")
}
