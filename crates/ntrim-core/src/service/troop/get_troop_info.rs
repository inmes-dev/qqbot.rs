use std::sync::Arc;
use anyhow::Error;
use crate::await_response;
use crate::bot::Bot;
use crate::commands::troop::GroupInfo;

impl Bot {
    pub async fn get_troop_info(self: &Arc<Self>, group_id: i64) -> Result<GroupInfo, Error> {
        let group_info_list = await_response!(tokio::time::Duration::from_secs(30), async {
            let rx = Bot::_get_troop_info(self, vec![group_id]).await;
            if let Some(rx) = rx {
                rx.await.map_err(|e| anyhow::Error::from(e))
            } else {
                Err(Error::msg("Unable to get_group_info: tcp connection exception"))
            }
        }, |value| {
            Ok(value)
        }, |e| {
            Err(e)
        })?.ok_or(Error::msg("GetTroopInfo result as null"))?;
        let group_info = group_info_list.first()
            .ok_or(Error::msg("GetTroopInfo result is empty"))?;
        Ok(group_info.clone())
    }
}