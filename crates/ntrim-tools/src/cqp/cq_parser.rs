use std::collections::HashMap;
use anyhow::{anyhow, Error};
use log::error;
use crate::cqp::{ * };

fn utf8_next_len(str: &[u8], offset: usize) -> usize {
    let c = str[offset];
    if c >= 0xfc {
        6
    } else if c >= 0xf8 {
        5
    } else if c >= 0xf0 {
        4
    } else if c >= 0xe0 {
        3
    } else if c >= 0xc0 {
        2
    } else if c > 0x0 {
        1
    } else {
        0
    }
}

pub fn parse_cq(data: &[u8]) -> Result<Vec<CQCode>, Error> {
    let mut start = false;
    let mut cache = String::new();
    let mut result = Vec::new();
    let mut cq_type = String::new();
    let mut key = String::new();
    let mut params = HashMap::new();

    let mut i = 0;
    while i < data.len() {
        let utf_char_len = utf8_next_len(data, i);
        if utf_char_len == 0 {
            continue;
        }
        let utf_char = &data[i..i+utf_char_len];
        let c = unsafe { std::str::from_utf8_unchecked(utf_char) };
        if c == "[" {
            if start {
                return Err(Error::msg("Illegal code"))
            } else {
                if !cache.is_empty() {
                    let text = CQCode::Text(cache
                        .replace("&#91;", "[")
                        .replace("&#93;", "]")
                        .replace("&amp;", "&")
                    );
                    result.push(text);
                    cache.clear();
                }
                let cq_flag = unsafe { std::str::from_utf8_unchecked(&data[i..i + 4]) };
                if cq_flag == "[CQ:" {
                    start = true;
                    i += 3;
                } else {
                    cache.push_str(c);
                }
            }
        } else if c == "=" {
            if start {
                if cache.is_empty() {
                    return Err(Error::msg("Illegal code"))
                }
                if key.is_empty() {
                    key = cache.clone();
                    cache.clear();
                } else {
                    cache.push_str(c);
                }
            } else {
                cache.push_str(c);
            }
        } else if c == "," {
            if start {
                if cache.is_empty() {
                    return Err(Error::msg("Illegal code"))
                }
                if cq_type.is_empty() {
                    cq_type = cache.clone();
                    cache.clear();
                } else {
                    if !key.is_empty() {
                        params.insert(key.clone(), cache.
                            replace("&#91;", "[")
                            .replace("&#93;", "]")
                            .replace("&#44;", ",")
                            .replace("&amp;", "&")
                        );
                        key.clear();
                        cache.clear();
                    }
                }
            } else {
                cache.push_str(c);
            }
        } else if c == "]" {
            if start {
                if !cache.is_empty() {
                    if !key.is_empty() {
                        params.insert(key.clone(), cache
                            .replace("&#91;", "[")
                            .replace("&#93;", "]")
                            .replace("&#44;", ",")
                            .replace("&amp;", "&")
                        );
                    } else {
                        cq_type = cache.clone();
                        cache.clear();
                    }
                    let cq_code = parse_special_cq(&cq_type, &params)?;
                    result.push(cq_code);
                    cq_type.clear();
                    params.clear();
                    key.clear();
                    cache.clear();
                    start = false;
                } else {
                    cache.push_str(c);
                }
            } else {
                cache.push_str(c);
            }
        } else {
            cache.push_str(c);
            i += utf_char_len - 1;
        }
        i += 1;
    }
    if !cache.is_empty() {
        let text = CQCode::Text(cache
            .replace("&#91;", "[")
            .replace("&#93;", "]")
            .replace("&amp;", "&")
        );
        result.push(text);
    }
    Ok(result)
}

pub(crate) fn parse_special_cq(flag: &str, params: &HashMap<String, String>) -> Result<CQCode, Error> {
    match flag {
        "text" => Ok(CQCode::Text(params.get("text").unwrap().to_owned())),
        "at" => Ok(CQCode::At(At::from(params)?)),
        "face" => Ok(CQCode::Face(Face::from(params)?)),
        "image" => Ok(CQCode::Image(Image::from(params)?)),
        "record" => Ok(CQCode::Record(Record::from(params)?)),
        "video" => Ok(CQCode::Video(Video::from(params)?)),
        "reply" => Ok(CQCode::Reply(Reply::from(params)?)),
        "poke" => Ok(CQCode::Poke(Poke::from(params)?)),
        "dice" => Ok(CQCode::NewDice(NewDice::from(params)?)),
        "rps" => Ok(CQCode::NewRPS(NewRPS::from(params)?)),
        "music" => {
            let music_type = params.get("type").ok_or(anyhow!("CustomMusic 缺少 'type' 参数"))?;
            if music_type == "custom" {
                Ok(CQCode::CustomMusic(CustomMusic::from(params)?))
            } else {
                Ok(CQCode::Music(Music::from(params)?))
            }
        },
        "share" => Ok(CQCode::Share(Share::from(params)?)),
        "location" => Ok(CQCode::Location(Location::from(params)?)),
        "weather" => Ok(CQCode::Weather(Weather::from(params)?)),
        "gift" => Ok(CQCode::Gift(Gift::from(params)?)),
        "basketball" => Ok(CQCode::Basketball(Basketball::from(params)?)),
        "bubble_face" => Ok(CQCode::BubbleFace(BubbleFace::from(params)?)),
        "touch" => Ok(CQCode::Touch(Touch::from(params)?)),
        &_ => {
            error!("Parse cqcode failed: unknown cq code, type: {}", flag);
            Err(anyhow!("Parse cqcode failed: unknown cq code, type: {}", flag))
        }
    }
}
