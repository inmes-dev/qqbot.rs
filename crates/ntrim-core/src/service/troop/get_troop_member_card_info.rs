use std::sync::Arc;
use anyhow::anyhow;
use crate::bot::Bot;
use crate::commands::troop::GroupMemberInfo;
use crate::{await_response, db};
use crate::db::PG_POOL;

impl Bot {
    pub async fn get_troop_member_card_info(self: &Arc<Bot>, group_id: i64, user_id: i64, refresh: Option<bool>) -> anyhow::Result<GroupMemberInfo> {
        #[cfg(feature = "sql")]
        if db::is_initialized() && !refresh.unwrap_or(false) {
            let pool = PG_POOL.get().unwrap();
            let group_member_info = GroupMemberInfo::query_member(pool, group_id, user_id).await?;
            return Ok(group_member_info)
        }
        let info = await_response!(tokio::time::Duration::from_secs(5), async {
            let rx = Bot::_get_group_member_card_info(self, group_id, user_id).await;
            if let Some(rx) = rx {
                rx.await.map_err(|e| anyhow::Error::from(e))
            } else {
                Err(anyhow::Error::msg("Unable to handle_get_group_member_info: tcp connection exception"))
            }
        }, |value| {
            Ok(value)
        }, |e| {
            Err(e)
        })?.ok_or(anyhow!("Failed to get troop member card info: timeout or wind ctrl"))?;
        return Ok(info);
    }
}