use std::sync::Arc;
use anyhow::Error;
use log::warn;
use crate::{await_response, db};
use crate::bot::Bot;
use crate::commands::troop::{GroupMemberInfo, GroupMemberPermission};
#[cfg(feature = "sql")]
use crate::db::{PG_POOL};

impl Bot {
    pub async fn get_troop_member_list(
        self: &Arc<Self>,
        group_id: i64,
        owner_uin: i64,
    ) -> Result<Vec<GroupMemberInfo>, Error> {
        let mut next_uin = 0;
        let mut list = Vec::new();
        loop {
            match await_response!(tokio::time::Duration::from_secs(10), async {
                let rx = Bot::_get_troop_member_list(self, group_id, next_uin).await;
                if let Some(rx) = rx {
                    rx.await.map_err(|e| Error::new(e))
                } else {
                    Err(Error::msg("Unable to get_troop_member_list: tcp connection exception"))
                }
            }, |value| {
                Ok(value)
            }, |e| {
                Err(e)
            }) {
                Ok(Some((mut rl, n))) => {
                    if rl.is_empty() {
                        return Err(Error::msg("GroupMemberListResponse.list"));
                    }
                    for m in rl.iter_mut() {
                        if m.uin == owner_uin {
                            m.permission = GroupMemberPermission::Owner;
                        }
                        m.group_code = group_id;
                    }
                    list.append(&mut rl);
                    next_uin = n;
                    if next_uin == 0 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(None) => {
                    warn!("get_troop_member_list: no more data");
                    break;
                }
            }
        }
        #[cfg(feature = "sql")]
        if db::is_initialized() {
            let list = list.clone();
            tokio::spawn(async move {
                let pool = PG_POOL.get().unwrap();
                for member in list.into_iter() {
                    GroupMemberInfo::insert(pool, member).await
                        .expect("Failed to insert group member info");
                }
            });
        }
        Ok(list)
    }

    #[cfg(feature = "sql")]
    pub async fn get_troop_member_list_from_cache(
        self: &Arc<Self>,
        group_id: i64
    ) -> Result<Vec<GroupMemberInfo>, Error> {
        let pool = PG_POOL.get().unwrap();
        GroupMemberInfo::query_by_group_id(pool, group_id).await
    }
}
