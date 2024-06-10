use anyhow::Error;
use sqlx::PgPool;
use crate::commands::troop::GroupInfo;

const TABLE_NAME: &'static str = "troop_list";

impl GroupInfo {
    pub async fn create_table(pool: &PgPool) -> Result<(), Error> {
        let exists: (bool,) = sqlx::query_as(format!("SELECT EXISTS ( \
            SELECT 1 \
            FROM information_schema.tables \
            WHERE TABLE_NAME = '{}' \
        )", TABLE_NAME).as_str()).fetch_one(pool).await?;
        if !exists.0 {
            sqlx::query(format!("CREATE TABLE {} ( \
                id BIGINT PRIMARY KEY, \
                name VARCHAR(255) NOT NULL, \
                uin BIGINT NOT NULL, \
                memo VARCHAR(255) NOT NULL, \
                owner BIGINT NOT NULL,\
                create_time BIGINT NOT NULL, \
                level INT NOT NULL, \
                member_count INT NOT NULL, \
                max_member_count INT NOT NULL, \
                shut_up_timestamp BIGINT NOT NULL, \
                my_shut_up_timestamp BIGINT NOT NULL, \
                last_msg_seq BIGINT NOT NULL \
            )", TABLE_NAME).as_str()).execute(pool).await?;
        }
        Ok(())
    }

    pub async fn insert(pool: &PgPool, group: GroupInfo) -> Result<(), Error> {
        sqlx::query(format!(r#"
            INSERT INTO "{}" ("id", "name", "uin", "memo", "owner", "create_time", "level", "member_count", "max_member_count", "shut_up_timestamp", "my_shut_up_timestamp", "last_msg_seq")
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT ("id") DO UPDATE SET
                "name" = EXCLUDED."name",
                "uin" = EXCLUDED."uin",
                "memo" = EXCLUDED."memo",
                "owner" = EXCLUDED."owner",
                "create_time" = EXCLUDED."create_time",
                "level" = EXCLUDED."level",
                "member_count" = EXCLUDED."member_count",
                "max_member_count" = EXCLUDED."max_member_count",
                "shut_up_timestamp" = EXCLUDED."shut_up_timestamp",
                "my_shut_up_timestamp" = EXCLUDED."my_shut_up_timestamp",
                "last_msg_seq" = EXCLUDED."last_msg_seq"
        "#, TABLE_NAME).as_str())
            .bind(group.code)
            .bind(&group.name)
            .bind(group.uin)
            .bind(&group.memo)
            .bind(group.owner_uin)
            .bind(group.group_create_time)
            .bind(group.group_level)
            .bind(group.member_count)
            .bind(group.max_member_count)
            .bind(group.shut_up_timestamp)
            .bind(group.my_shut_up_timestamp)
            .bind(group.last_msg_seq)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<GroupInfo, Error> {
        let group = sqlx::query_as::<_, GroupInfo>(format!(
            "SELECT id, name, uin, memo, owner, create_time, level, member_count, max_member_count, shut_up_timestamp, my_shut_up_timestamp, last_msg_seq FROM {} WHERE id = $1", TABLE_NAME
        ).as_str())
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(group)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<GroupInfo>, Error> {
        let groups = sqlx::query_as::<_, GroupInfo>(format!(
            "SELECT id, name, uin, memo, owner, create_time, level, member_count, max_member_count, shut_up_timestamp, my_shut_up_timestamp, last_msg_seq FROM {}", TABLE_NAME
        ).as_str())
            .fetch_all(pool)
            .await?;
        Ok(groups)
    }
}