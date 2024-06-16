use std::sync::Arc;
use anyhow::Error;
use bytes::Bytes;
use log::{error, warn};
use crate::bot::Bot;
use crate::{await_response, db};
use crate::commands::troop::GroupInfo;
#[cfg(feature = "sql")]
use crate::db::{PG_POOL};

impl Bot {
    pub async fn get_troop_list(self: &Arc<Self>, refresh: bool) -> Result<Vec<GroupInfo>, Error> {
        #[cfg(feature = "sql")]
        if !refresh && db::is_initialized() {
            // 数据库支持打开且不需要刷新则从数据库获取
            let pool = PG_POOL.get().unwrap();
            match GroupInfo::get_all(pool).await {
                Ok(result) => {
                    return Ok(result);
                }
                Err(e) => {
                    error!("Failed to get group list from database: {}", e);
                }
            }
        }
        let mut vec_cookie = Bytes::new();
        let mut groups = Vec::new();
        loop {
            match await_response!(tokio::time::Duration::from_secs(5), async {
                let rx = Bot::_get_troop_list(self, &vec_cookie).await;
                if let Some(rx) = rx {
                    rx.await.map_err(|e| Error::new(e))
                } else {
                    Err(Error::msg("Unable to get_troop_list: tcp connection exception"))
                }
            }, |value| {
                Ok(value)
            }, |e| {
                Err(e)
            }) {
                Ok(Some((mut g, c))) => {
                    groups.append(&mut g);
                    if c.is_empty() {
                        break
                    }
                    vec_cookie = c;
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(None) => {
                    warn!("get_troop_list: no more data");
                    break;
                }
            }
        }
        #[cfg(feature = "sql")]
        if db::is_initialized() {
            let groups = groups.clone();
            tokio::spawn(async move {
                let pool = PG_POOL.get().unwrap();
                for group_info in groups.into_iter() {
                    GroupInfo::insert(pool, group_info).await
                        .expect("Failed to insert group info");
                }
            });
        }
        Ok(groups)
    }
}