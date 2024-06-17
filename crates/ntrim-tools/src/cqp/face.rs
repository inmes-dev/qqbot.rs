use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Error};

#[derive(Debug, Clone, Default)]
pub struct Face {
    pub id: u32,
    pub big: bool,
    pub result: u32,
    pub extend: bool,
}

impl Face {
    pub fn new(id: u32) -> Self {
        Face {
            id,
            big: false,
            result: 0,
            extend: false
        }
    }

    pub fn new_big_face(id: u32, result: u32) -> Self {
        Face {
            id,
            big: true,
            result,
            extend: false
        }
    }

    pub fn with_result(id: u32, result: u32) -> Self {
        Face {
            id,
            big: true,
            result,
            extend: false
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.big {
            write!(f, "[CQ:face,id={},big={},result={}]", self.id, self.big, self.result)
        } else {
            write!(f, "[CQ:face,id={}]", self.id)
        }
    }
}

impl Face {
    pub(crate) fn from(params: &HashMap<String, String>) -> Result<Self, Error> {
        let id = params.get("id").ok_or(anyhow!("Face 缺少 'id' 参数"))?.parse::<u32>()?;
        let big = params.get("big").map(|s| s.parse::<bool>().unwrap_or(false)).unwrap_or(false);
        let result = params.get("result").map(|s| s.parse::<u32>().unwrap_or(0)).unwrap_or(0);
        Ok(Face {
            id,
            big,
            result,
            extend: false
        })
    }
}