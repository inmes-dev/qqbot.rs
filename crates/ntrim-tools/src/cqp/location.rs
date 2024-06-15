use std::collections::HashMap;
use anyhow::{anyhow, Error};

#[derive(Debug, Default)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:location,lat={},lon={}]", self.lat, self.lon)?;
        if let Some(title) = &self.title {
            write!(f, ",title={}", title)?;
        }
        if let Some(content) = &self.content {
            write!(f, ",content={}", content)?;
        }
        Ok(())
    }
}

impl Location {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let lat = params.get("lat").ok_or(anyhow!("Location 缺少 'lat' 参数"))?.parse::<f64>()?;
        let lon = params.get("lon").ok_or(anyhow!("Location 缺少 'lon' 参数"))?.parse::<f64>()?;
        let title = params.get("title").map(|s| s.to_string());
        let content = params.get("content").map(|s| s.to_string());
        Ok(Location {
            lat,
            lon,
            title,
            content,
        })
    }
}