use std::collections::HashMap;
use anyhow::{anyhow, Error};
use serde_json::Value;
use crate::cqp::cq_parser::parse_special_cq;
use crate::cqp::CQCode;

fn json_to_hashmap(value: &Value) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    if let Value::Object(map) = value {
        for (key, val) in map.iter() {
            if val.is_string() {
                hashmap.insert(key.clone(), val.as_str().unwrap().to_string());
            } else {
                hashmap.insert(key.clone(), val.to_string());
            }
        }
    }
    hashmap
}

pub fn parse_single_segment(value: Value) -> Result<CQCode, Error> {
    if !value.is_object() {
        return Err(anyhow!("这种奇怪的输入不是一个正常的消息段吧？"));
    }
    let flag = value.get("type").ok_or(anyhow!("消息段缺少 'type' 字段"))?.as_str().unwrap();
    let data = value.get("data").ok_or(anyhow!("消息段缺少 'data' 字段"))?;
    let params = json_to_hashmap(data);
    let cq = parse_special_cq(flag, &params)?;
    return Ok(cq);
}

pub fn parse_segments(value: Value) -> Result<Vec<CQCode>, Error> {
    if !value.is_array() {
        return Err(anyhow!("这种奇怪的输入不是正常的消息段吧？"));
    }
    let mut result = Vec::new();
    if let Value::Array(array) = value {
        for item in array {
            let cq = parse_single_segment(item)?;
            result.push(cq);
        }
    }
    return Ok(result);
}

#[test]
fn test_json_to_hashmap() {
    let json_value = serde_json::json!({
        "name": "伏秋洛",
        "age": 18,
        "height": 1.68,
        "active": true,
        "sex": "female"
    });

    let result = json_to_hashmap(&json_value);
    let expected: HashMap<String, String> = [
        ("name".to_string(), "伏秋洛".to_string()),
        ("age".to_string(), "18".to_string()),
        ("height".to_string(), "1.68".to_string()),
        ("active".to_string(), "true".to_string()),
        ("sex".to_string(), "female".to_string())
    ].iter().cloned().collect();

    assert_eq!(result, expected);
}