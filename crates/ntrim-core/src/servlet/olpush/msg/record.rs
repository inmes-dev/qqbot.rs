use std::fmt::Display;
use sqlx::FromRow;
use ntrim_tools::cqp::CQCode;

pub enum Contact {
    //        name   uin   uid
    Group(   String, u64),
    Friend(  String, u64, String),
    Stranger(String, u64, String),
}

pub struct MessageRecord {
    pub contact: Contact,
    pub sender_id: u64,
    pub sender_uid: String,
    pub sender_nick: String,
    pub sender_unique_title: String,
    pub msg_time: i64,
    pub msg_seq: u64,
    pub msg_uid: u64,
    pub elements: Vec<CQCode>,
}

impl Display for MessageRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contact = match &self.contact {
            Contact::Group(group_name, group_id) => ("群", group_name, group_id),
            Contact::Friend(user_name, uin, _) => ("好友", user_name, uin),
            Contact::Stranger(user_name, uin, _) => ("陌生人", user_name, uin),
        };
        let raw_msg = self.elements.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
        write!(f, "{}消息 [{}({})] {}({}): {}", contact.0, contact.1, contact.2, self.sender_nick, self.sender_uid, raw_msg)
    }
}