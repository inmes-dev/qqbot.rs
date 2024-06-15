use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct Share {
    pub url: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub file: Option<String>,
}

impl std::fmt::Display for Share {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:share,url={}]", self.url)?;
        if let Some(title) = &self.title {
            write!(f, ",title={}", title)?;
        }
        if let Some(content) = &self.content {
            write!(f, ",content={}", content)?;
        }
        if let Some(image) = &self.image {
            write!(f, ",image={}", image)?;
        }
        if let Some(file) = &self.file {
            write!(f, ",file={}", file)?;
        }
        Ok(())
    }
}

impl Share {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let url = params.get("url").ok_or(anyhow!("Share 缺少 'url' 参数"))?.to_string();
        let title = params.get("title").map(|s| s.to_string());
        let content = params.get("content").map(|s| s.to_string());
        let image = params.get("image").map(|s| s.to_string());
        let file = params.get("file").map(|s| s.to_string());
        Ok(Share {
            url,
            title,
            content,
            image,
            file,
        })
    }
}