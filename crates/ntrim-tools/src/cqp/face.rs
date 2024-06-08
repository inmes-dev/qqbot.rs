use std::fmt::{Display, Formatter};
use crate::cqp::SpecialCQCode;

pub struct Face {
    pub id: u32,
    pub big: bool,
    pub result: u32
}

impl Face {
    pub fn new(id: u32) -> Self {
        Face {
            id,
            big: false,
            result: 0
        }
    }

    pub fn new_big_face(id: u32) -> Self {
        Face {
            id,
            big: true,
            result: 0
        }
    }

    pub fn with_result(id: u32, result: u32) -> Self {
        Face {
            id,
            big: true,
            result
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:face,id={},big={},result={}]", self.id, self.big, self.result)
    }
}

impl SpecialCQCode for Face {
    fn get_type(&self) -> String {
        "face".to_string()
    }
}