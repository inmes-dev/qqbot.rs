use std::collections::HashMap;
use anyhow::Error;

#[derive(Debug, Default)]
pub struct Weather {
    pub city: Option<String>,
    pub code: Option<String>,
}

impl std::fmt::Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(city) = &self.city {
            write!(f, "[CQ:weather,city={}]", city)
        } else if let Some(code) = &self.code {
            write!(f, "[CQ:weather,code={}]", code)
        } else {
            write!(f, "[CQ:weather]")
        }
    }
}

impl Weather {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let city = params.get("city").map(|s| s.to_string());
        let code = params.get("code").map(|s| s.to_string());
        Ok(Weather {
            city,
            code,
        })
    }
}