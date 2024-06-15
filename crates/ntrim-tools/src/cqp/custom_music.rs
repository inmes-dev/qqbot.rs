use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct CustomMusic {
    pub music_type: String,
    pub url: String,
    pub audio: String,
    pub title: String,
    pub singer: String,
    pub image: Option<String>,
}

impl std::fmt::Display for CustomMusic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(image) = &self.image {
            write!(f, "[CQ:music,type={},url={},audio={},title={},singer={},image={}]", self.music_type, self.url, self.audio, self.title, self.singer, image)
        } else {
            write!(f, "[CQ:music,type={},url={},audio={},title={},singer={}]", self.music_type, self.url, self.audio, self.title, self.singer)
        }
    }
}


impl CustomMusic {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let music_type = params.get("type").ok_or(anyhow!("CustomMusic 缺少 'type' 参数"))?;
        let url = params.get("url").ok_or(anyhow!("CustomMusic 缺少 'url' 参数"))?;
        let audio = params.get("audio").ok_or(anyhow!("CustomMusic 缺少 'audio' 参数"))?;
        let title = params.get("title").ok_or(anyhow!("CustomMusic 缺少 'title' 参数"))?;
        let singer = params.get("singer").ok_or(anyhow!("CustomMusic 缺少 'singer' 参数"))?;
        let image = params.get("image").map(|s| s.to_string());
        Ok(CustomMusic {
            music_type: music_type.to_string(),
            url: url.to_string(),
            audio: audio.to_string(),
            title: title.to_string(),
            singer: singer.to_string(),
            image,
        })
    }
}