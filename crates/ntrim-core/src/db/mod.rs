pub mod simple_record;
pub mod message_record;
pub mod group_list;
mod group_member_list;
mod friend_list;

use std::sync::OnceLock;
use sqlx::{Acquire, PgPool};
use sqlx::postgres::PgPoolOptions;
use crate::commands::friend::{FriendListResponse};
use crate::commands::troop::{GroupInfo, GroupMemberInfo};
pub use crate::db::simple_record::SimpleMessageRecord;
use crate::MessageRecord;

pub static PG_POOL: OnceLock<PgPool> = OnceLock::new();

pub fn is_initialized() -> bool {
    PG_POOL.get().is_some()
}

pub async fn initialize_pool(addr: &str) {
    let pool = PgPoolOptions::new()
        .max_connections(std::env::var("SQL_MAX_CONNECTIONS").map_or(5, |v| v.parse().unwrap()))
        .connect(addr)
        .await
        .expect("Failed to create PgPool");
    PG_POOL.set(pool).expect("Failed to set PgPool");
}

async fn check_database_connection() -> Result<(), anyhow::Error> {
    let pool = PG_POOL.get().unwrap();
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

pub async fn ensure_table_exists() -> Result<(), anyhow::Error> {
    check_database_connection().await?;
    let pool = PG_POOL.get().unwrap();
    tokio::try_join!(
        SimpleMessageRecord::create_table(pool),
        MessageRecord::create_table(pool),
        GroupInfo::create_table(pool),
        GroupMemberInfo::create_table(pool),
        FriendListResponse::create_table(pool),
    )?;
    Ok(())
}