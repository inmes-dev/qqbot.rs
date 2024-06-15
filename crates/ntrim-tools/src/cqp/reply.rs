use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Reply {
    pub id: i64,
}

impl Display for Reply {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:reply,id={}]", self.id)
    }
}

impl Reply {
    pub(crate) fn from(params: &std::collections::HashMap<String, String>) -> Result<Self, anyhow::Error> {
        let id = params.get("id").ok_or(anyhow::anyhow!("Reply 缺少 'id' 参数"))?.parse::<i64>()?;
        Ok(Reply {
            id,
        })
    }
}