use std::sync::Arc;
use anyhow::Error;
use log::warn;
use crate::await_response;
use crate::bot::Bot;
use crate::commands::friend::FriendListResponse;

impl Bot {
    pub async fn get_friend_list(self: &Arc<Self>, refresh: bool) -> Result<FriendListResponse, Error> {
        let bot_id = self.unique_id;
        #[cfg(feature = "sql")]
        if crate::db::is_initialized() && !refresh {
            let pool = crate::db::PG_POOL.get().unwrap();
            let friend_list = FriendListResponse::get_friend_list(pool, bot_id).await?;
            let group_list = FriendListResponse::get_group_list(pool, bot_id).await?;
            let cnt = friend_list.len();
            return Ok(FriendListResponse {
                friends: friend_list,
                friend_groups: group_list.into_iter().map(|g| (g.group_id, g)).collect(),
                total_count: cnt as i16,
                online_friend_count: 0,
            });
        }
        let mut output = FriendListResponse::default();
        loop {
            match await_response!(tokio::time::Duration::from_secs(5), async {
                let rx = Bot::_get_friend_list(self, output.friends.len() as i16, 150, 0, 0).await;
                if let Some(rx) = rx {
                    rx.await.map_err(|e| Error::new(e))
                } else {
                    Err(Error::msg("Unable to get_friend_list: tcp connection exception"))
                }
            }, |value| {
                Ok(value)
            }, |e| {
                Err(e)
            }) {
                Ok(Some(resp)) => {
                    output.friend_groups.extend(resp.friend_groups);
                    output.friends.extend(resp.friends);
                    output.total_count = resp.total_count;
                    if output.friends.len() as i16 >= resp.total_count {
                        break;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(None) => {
                    warn!("get_friend_list: no more data");
                    break;
                }
            }
        }
        #[cfg(feature = "sql")]
        if crate::db::is_initialized() {
            let mut output = output.clone();
            tokio::spawn(async move {
                let pool = crate::db::PG_POOL.get().unwrap();
                for friend_info in output.friends.into_iter() {
                    FriendListResponse::insert(pool, bot_id, friend_info).await
                        .expect("Failed to insert friend");
                }
                for (_, group_info) in output.friend_groups.into_iter() {
                    FriendListResponse::insert_group(pool, bot_id, group_info).await
                        .expect("Failed to insert friend group");
                }
            });
        }
        Ok(output)
    }
}