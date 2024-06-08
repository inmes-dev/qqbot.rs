use std::collections::HashMap;
use anyhow::Error;
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
                    result.push(CQCode::Special(cq_code));
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

fn parse_special_cq(flag: &str, params: &HashMap<String, String>) -> Result<Box<dyn SpecialCQCode>, Error> {
    match flag {
        "at" => Ok(Box::new(At {
            qq: params.get("qq").ok_or_else(|| Error::msg("qq is none"))?.parse()?,
            #[cfg(feature = "extend_cqcode")]
            content: params.get("content").ok_or_else(|| Error::msg("content is none"))?.clone(),
        })),
        &_ => Err(Error::msg(format!("Parse cqcode failed: unknown cq code, type: {}", flag)))
    }
}
