use std::fmt::format;
use anyhow::Error;
use sqlx::{PgPool, Row};
use crate::commands::troop::GroupMemberInfo;
use crate::commands::troop::GroupMemberPermission::{Administrator, Member, Owner};

const TABLE_NAME: &'static str = "group_member_list";

impl GroupMemberInfo {
    pub async fn create_table(pool: &PgPool) -> Result<(), Error> {
        let exists: (bool,) = sqlx::query_as(format!("SELECT EXISTS ( \
            SELECT 1 \
            FROM information_schema.tables \
            WHERE TABLE_NAME = '{}' \
        )", TABLE_NAME).as_str()).fetch_one(pool).await?;
        if !exists.0 {
            sqlx::query(format!("CREATE TABLE {} ( \
                id SERIAL PRIMARY KEY, \
                group_id BIGINT NOT NULL, \
                uin BIGINT NOT NULL, \
                gender SMALLINT NOT NULL, \
                nick_name VARCHAR(255) NOT NULL, \
                card_name VARCHAR(255) NOT NULL, \
                level SMALLINT NOT NULL, \
                join_time BIGINT NOT NULL,\
                last_speak_time BIGINT NOT NULL,\
                special_title VARCHAR(255) NOT NULL, \
                special_title_expire_time BIGINT NOT NULL, \
                shut_up_timestamp BIGINT NOT NULL, \
                permission INT NOT NULL, \
                uid VARCHAR(255) NOT NULL, \
                honor int[], \
                UNIQUE (group_id, uin) \
            )", TABLE_NAME).as_str()).execute(pool).await?;
        }
        Ok(())
    }

    pub async fn insert(pool: &PgPool, info: GroupMemberInfo) -> Result<(), Error> {
        sqlx::query(format!(r#"INSERT INTO {} (
                group_id, uin, gender, nick_name, card_name, level,
                join_time, last_speak_time, special_title, special_title_expire_time,
                shut_up_timestamp, permission, uid, honor
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (group_id, uin)
            DO UPDATE SET
                gender = EXCLUDED.gender,
                nick_name = EXCLUDED.nick_name,
                card_name = EXCLUDED.card_name,
                level = EXCLUDED.level,
                join_time = EXCLUDED.join_time,
                last_speak_time = EXCLUDED.last_speak_time,
                special_title = EXCLUDED.special_title,
                special_title_expire_time = EXCLUDED.special_title_expire_time,
                shut_up_timestamp = EXCLUDED.shut_up_timestamp,
                permission = EXCLUDED.permission,
                honor = EXCLUDED.honor
        "#, TABLE_NAME).as_str())
            .bind(info.group_code)
            .bind(info.uin)
            .bind(info.gender)
            .bind(info.nickname)
            .bind(info.card_name)
            .bind(info.level)
            .bind(info.join_time)
            .bind(info.last_speak_time)
            .bind(info.special_title)
            .bind(info.special_title_expire_time)
            .bind(info.shut_up_timestamp)
            .bind(info.permission as i32)
            .bind(info.uid)
            .bind(&info.honor[..])
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn query_by_group_id(pool: &PgPool, group_id: i64) -> Result<Vec<GroupMemberInfo>, Error> {
        let rows = sqlx::query(format!("SELECT * FROM {} WHERE group_id = $1", TABLE_NAME).as_str())
            .bind(group_id)
            .fetch_all(pool)
            .await?;

        let mut members = Vec::new();

        for row in rows {
            let honor = row.get::<Vec<i32>, _>("honor");
            members.push(GroupMemberInfo {
                group_code: row.get("group_id"),
                uin: row.get("uin"),
                uid: row.get("uid"),
                permission: match row.get::<i32, _>("permission") {
                    1 => Owner,
                    2 => Administrator,
                    _ => Member
                },
                gender: row.get("gender"),
                nickname: row.get("nick_name"),
                card_name: row.get("card_name"),
                level: row.get("level"),
                join_time: row.get("join_time"),
                last_speak_time: row.get("last_speak_time"),
                special_title: row.get("special_title"),
                special_title_expire_time: row.get("special_title_expire_time"),
                shut_up_timestamp: row.get("shut_up_timestamp"),
                honor,
                ..Default::default()
            });
        }

        Ok(members)
    }
}