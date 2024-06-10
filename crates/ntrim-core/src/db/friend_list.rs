use anyhow::Error;
use sqlx::PgPool;

const TABLE_NAME_FRIEND: &'static str = "friend_list";
const TABLE_NAME_FRIEND_GROUP: &'static str = "friend_group_list";

impl crate::commands::friend::FriendListResponse {
    pub async fn create_table(pool: &PgPool) -> Result<(), Error> {
        let exists: (bool,) = sqlx::query_as(format!("SELECT EXISTS ( \
            SELECT 1 \
            FROM information_schema.tables \
            WHERE TABLE_NAME = '{}' \
        )", TABLE_NAME_FRIEND).as_str()).fetch_one(pool).await?;
        if !exists.0 {
            sqlx::query(format!("CREATE TABLE {} ( \
                id SERIAL PRIMARY KEY, \
                bot BIGINT NOT NULL, \
                uin BIGINT NOT NULL, \
                name VARCHAR(255) NOT NULL, \
                remark VARCHAR(255) NOT NULL, \
                face_id SMALLINT NOT NULL, \
                group_id SMALLINT NOT NULL, \
                UNIQUE (bot, uin) \
            )", TABLE_NAME_FRIEND).as_str()).execute(pool).await?;
        }

        let exists: (bool,) = sqlx::query_as(format!("SELECT EXISTS ( \
            SELECT 1 \
            FROM information_schema.tables \
            WHERE TABLE_NAME = '{}' \
        )", TABLE_NAME_FRIEND_GROUP).as_str()).fetch_one(pool).await?;
        if !exists.0 {
            sqlx::query(format!("CREATE TABLE {} ( \
                id SERIAL PRIMARY KEY, \
                bot BIGINT NOT NULL, \
                group_id SMALLINT NOT NULL, \
                group_name VARCHAR(255) NOT NULL, \
                friend_count INT NOT NULL, \
                online_friend_count INT NOT NULL, \
                seq_id SMALLINT NOT NULL, \
                UNIQUE (bot, group_id) \
            )", TABLE_NAME_FRIEND_GROUP).as_str()).execute(pool).await?;
        }
        Ok(())
    }

    pub async fn insert(pool: &PgPool, bot_id: i64, friend: crate::commands::friend::FriendInfo) -> Result<(), Error> {
        sqlx::query(format!(r#"
            INSERT INTO "{}" ("bot", "uin", "name", "remark", "face_id", "group_id")
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT ("bot", "uin") DO UPDATE SET
                "name" = EXCLUDED."name",
                "remark" = EXCLUDED."remark",
                "face_id" = EXCLUDED."face_id",
                "group_id" = EXCLUDED."group_id"
        "#, TABLE_NAME_FRIEND).as_str())
            .bind(bot_id)
            .bind(friend.uin)
            .bind(&friend.nick)
            .bind(&friend.remark)
            .bind(friend.face_id)
            .bind(friend.group_id)
            .execute(pool).await?;
        Ok(())
    }

    pub async fn insert_group(pool: &PgPool, bot_id: i64, group: crate::commands::friend::FriendGroupInfo) -> Result<(), Error> {
        sqlx::query(format!(r#"
            INSERT INTO "{}" ("bot", "group_id", "group_name", "friend_count", "online_friend_count", "seq_id")
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT ("bot", "group_id") DO UPDATE SET
                "group_name" = EXCLUDED."group_name",
                "friend_count" = EXCLUDED."friend_count",
                "online_friend_count" = EXCLUDED."online_friend_count",
                "seq_id" = EXCLUDED."seq_id"
        "#, TABLE_NAME_FRIEND_GROUP).as_str())
            .bind(bot_id)
            .bind(group.group_id)
            .bind(&group.group_name)
            .bind(group.friend_count)
            .bind(group.online_friend_count)
            .bind(group.seq_id)
            .execute(pool).await?;
        Ok(())
    }

    pub async fn get_friend_list(pool: &PgPool, bot_id: i64) -> Result<Vec<crate::commands::friend::FriendInfo>, Error> {
        let friends = sqlx::query_as(format!("SELECT * FROM {} WHERE bot = $1", TABLE_NAME_FRIEND).as_str())
            .bind(bot_id)
            .fetch_all(pool)
            .await?;
        Ok(friends)
    }

    pub async fn get_group_list(pool: &PgPool, bot_id: i64) -> Result<Vec<crate::commands::friend::FriendGroupInfo>, Error> {
        let groups = sqlx::query_as(format!("SELECT * FROM {} WHERE bot = $1", TABLE_NAME_FRIEND_GROUP).as_str())
            .bind(bot_id)
            .fetch_all(pool)
            .await?;
        Ok(groups)
    }

    pub async fn get_friend_group_info(pool: &PgPool, bot_id: i64, group_id: i16) -> Result<crate::commands::friend::FriendGroupInfo, Error> {
        let group = sqlx::query_as(format!("SELECT * FROM {} WHERE bot = $1 AND group_id = $2", TABLE_NAME_FRIEND_GROUP).as_str())
            .bind(bot_id)
            .bind(group_id)
            .fetch_one(pool)
            .await?;
        Ok(group)
    }
}