use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub mod request_download_rkey;
pub mod request_upload_resource;
mod request_upload_ukey;

pub const SCENE_UNKNOWN: u32 = 0;
pub const SCENE_C2C: u32 = 1;
pub const SCENE_GROUP: u32 = 2;
pub const SCENE_CHANNEL: u32 = 3;

pub const SUB_FILE_TYPE_PIC: u32 = 0;

pub fn next_rich_media_seq(uin: i64) -> u32 {
    static MAP_RICH_MEDIA: Lazy<Mutex<HashMap<i64, AtomicU32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
    let mut request_id = MAP_RICH_MEDIA.lock().unwrap();
    let seq = request_id.entry(uin).or_insert(AtomicU32::new(10));
    if seq.load(std::sync::atomic::Ordering::Relaxed) >= 0x8000 {
        seq.store(1, std::sync::atomic::Ordering::Relaxed);
    }
    seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}