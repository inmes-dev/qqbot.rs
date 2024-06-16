pub mod cq_parser;
pub mod at;
pub mod face;
pub mod image;
pub mod bubble_face;
mod reply;
mod record;
mod video;
mod basketball;
mod new_rps;
mod new_dice;
mod poke;
mod touch;
mod music;
mod weather;
mod location;
mod share;
mod gift;
mod custom_music;
mod segment_parser;

pub use crate::cqp::at::At;
pub use crate::cqp::face::Face;
pub use crate::cqp::image::Image;
pub use crate::cqp::bubble_face::BubbleFace;
pub use crate::cqp::record::Record;
pub use crate::cqp::reply::Reply;
pub use crate::cqp::video::Video;
pub use crate::cqp::basketball::Basketball;
pub use crate::cqp::new_rps::NewRPS;
pub use crate::cqp::new_dice::NewDice;
pub use crate::cqp::poke::Poke;
pub use crate::cqp::touch::Touch;
pub use crate::cqp::music::Music;
pub use crate::cqp::weather::Weather;
pub use crate::cqp::location::Location;
pub use crate::cqp::share::Share;
pub use crate::cqp::gift::Gift;
pub use crate::cqp::custom_music::CustomMusic;

pub use cq_parser::parse_cq;
pub use segment_parser::parse_segments;
pub use segment_parser::parse_single_segment;
use std::fmt::Display;
use prost::Message;


pub enum CQCode {
    Text(String),
    At(At),
    Face(Face),
    Image(Image),
    BubbleFace(BubbleFace),
    Reply(Reply),
    Record(Record),
    Video(Video),
    Basketball(Basketball),
    NewRPS(NewRPS),
    NewDice(NewDice),
    Poke(Poke),  // 添加这一行
    Touch(Touch), // 添加这一行
    Music(Music),
    Weather(Weather),
    Location(Location),
    Share(Share),
    Gift(Gift),
    CustomMusic(CustomMusic),
}

impl Display for CQCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CQCode::Text(text) => write!(f, "{}", text),
            CQCode::At(at) => write!(f, "{}", at),
            CQCode::Face(face) => write!(f, "{}", face),
            CQCode::Image(image) => write!(f, "{}", image),
            CQCode::BubbleFace(bubble_face) => write!(f, "{}", bubble_face),
            CQCode::Reply(reply) => write!(f, "{}", reply),
            CQCode::Record(record) => write!(f, "{}", record),
            CQCode::Video(video) => write!(f, "{}", video),
            CQCode::Basketball(basketball) => write!(f, "{}", basketball),
            CQCode::NewRPS(new_rps) => write!(f, "{}", new_rps),
            CQCode::NewDice(new_dice) => write!(f, "{}", new_dice),
            CQCode::Poke(poke) => write!(f, "{}", poke),
            CQCode::Touch(touch) => write!(f, "{}", touch),
            CQCode::Music(music) => write!(f, "{}", music),
            CQCode::Weather(weather) => write!(f, "{}", weather),
            CQCode::Location(location) => write!(f, "{}", location),
            CQCode::Share(share) => write!(f, "{}", share),
            CQCode::Gift(gift) => write!(f, "{}", gift),
            CQCode::CustomMusic(custom_music) => write!(f, "{}", custom_music),
        }
    }
}

fn encode_cq_code_param(cq: &str) -> String {
    cq.replace("&", "&amp;")
        .replace("[", "&#91;")
        .replace("]", "&#93;")
        .replace(",", "&#44;")
}
