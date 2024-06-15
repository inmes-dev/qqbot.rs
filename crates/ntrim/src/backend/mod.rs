use std::sync::Arc;
use dashmap::DashMap;
use once_cell::sync::Lazy;

#[cfg(feature = "onebot")]
pub mod onebot;

#[cfg(feature = "kritor")]
pub mod kritor;

pub(crate) static UID_UIN_MAP: Lazy<Arc<DashMap<i64, String>>> = Lazy::new(|| Arc::new(DashMap::new()));
