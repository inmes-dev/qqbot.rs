use std::sync::Arc;
use anyhow::Error;
use bytes::Bytes;
use prost::Message;
use serde_json::from_slice;
use sqlx::{PgPool, Row};
use crate::bot::Bot;
use crate::pb::msg::RichText;
use crate::servlet::olpush::msg::{Contact, MessageRecord};

const TABLE_NAME: &'static str = "messages";

impl MessageRecord {
    pub async fn create_table(pool: &PgPool) -> Result<(), Error> {
        let exists: (bool,) = sqlx::query_as(format!("SELECT EXISTS ( \
            SELECT 1 \
            FROM information_schema.tables \
            WHERE table_schema = 'public' AND TABLE_NAME = '{}' \
        )", TABLE_NAME).as_str()).fetch_one(pool).await?;
        if !exists.0 {
            sqlx::query(format!("CREATE TABLE {} (\
                id SERIAL PRIMARY KEY, \
                contact_type VARCHAR(50) NOT NULL, \
                contact_name VARCHAR(255), \
                contact_uin BIGINT, \
                contact_uid VARCHAR(255), \
                sender_id BIGINT NOT NULL, \
                sender_uid VARCHAR(255) NOT NULL, \
                sender_nick VARCHAR(255) NOT NULL, \
                sender_unique_title VARCHAR(255) NOT NULL, \
                msg_time BIGINT NOT NULL, \
                msg_seq BIGINT NOT NULL, \
                msg_uid BIGINT NOT NULL UNIQUE, \
                receiver BIGINT NOT NULL, \
                elements BYTEA \
            )", TABLE_NAME).as_str()).execute(pool).await?;
        }
        Ok(())
    }

    pub async fn insert(pool: &PgPool, bot: &Arc<Bot>, message: &MessageRecord, raw_elems: Vec<u8>) -> Result<(), Error> {
        let (r#type, name, id, uid) = match &message.contact {
            Contact::Group(name, id) =>                 ("group",    name, *id as i64, ""),
            Contact::Friend(name, id, uid) =>   ("friend",   name, *id as i64, uid.as_str()),
            Contact::Stranger(name, id, uid) => ("stranger", name, *id as i64, uid.as_str())
        };
        sqlx::query(format!(r#"
            INSERT INTO "public"."{}" ("contact_type", "contact_name", "contact_uin", "contact_uid", "sender_id", "sender_uid", "sender_nick", "sender_unique_title", "msg_time", "msg_seq", "msg_uid", "receiver", "elements")
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT ("msg_uid") DO UPDATE SET
                "contact_type" = EXCLUDED."contact_type",
                "contact_name" = EXCLUDED."contact_name",
                "contact_uin" = EXCLUDED."contact_uin",
                "contact_uid" = EXCLUDED."contact_uid",
                "sender_id" = EXCLUDED."sender_id",
                "sender_uid" = EXCLUDED."sender_uid",
                "sender_nick" = EXCLUDED."sender_nick",
                "sender_unique_title" = EXCLUDED."sender_unique_title",
                "msg_time" = EXCLUDED."msg_time",
                "msg_seq" = EXCLUDED."msg_seq",
                "msg_uid" = EXCLUDED."msg_uid",
                "receiver" = EXCLUDED."receiver",
                "elements" = EXCLUDED."elements"
        "#, TABLE_NAME).as_str())
            .bind(r#type)
            .bind(name)
            .bind(id)
            .bind(uid)
            .bind(message.sender_id as i64)
            .bind(&message.sender_uid)
            .bind(&message.sender_nick)
            .bind(&message.sender_unique_title)
            .bind(&message.msg_time)
            .bind(message.msg_seq as i64)
            .bind(message.msg_uid as i64)
            .bind(bot.unique_id as i64)
            .bind(raw_elems)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_message_by_uid(pool: &PgPool, bot: &Arc<Bot>, msg_uid: u64) -> Result<MessageRecord, Error> {
        let row = sqlx::query(
            r#"
        SELECT
            contact_type,
            contact_name,
            contact_uin,
            contact_uid,
            sender_id,
            sender_uid,
            sender_nick,
            sender_unique_title,
            msg_time,
            msg_seq,
            msg_uid,
            receiver,
            elements
        FROM "public"."TABLE_NAME"
        WHERE msg_uid = $1 AND receiver = $2
        "#,
        )
            .bind(msg_uid as i64)
            .bind(bot.unique_id as i64)
            .fetch_one(pool)
            .await?;

        let name = row.get("contact_name");
        let id = row.get::<i64, _>("contact_uin") as u64;
        let uid = row.try_get("contact_uid").unwrap_or("".to_string());

        let contact = match row.get::<String, _>("contact_type").as_str() {
            "group" => Contact::Group(name, id),
            "friend" => Contact::Friend(name, id, uid),
            "stranger" => Contact::Stranger(name, id, uid),
            _ => return Err(Error::msg("unknown msg type")),
        };

        let mut record = MessageRecord {
            contact,
            sender_id: row.get::<i64, _>("sender_id") as u64,
            sender_uid: row.get("sender_uid"),
            sender_nick: row.get("sender_nick"),
            sender_unique_title: row.get("sender_unique_title"),
            msg_time: row.get("msg_time"),
            msg_seq: row.get::<i64, _>("msg_seq") as u64,
            msg_uid: row.get::<i64, _>("msg_uid") as u64,
            elements: Vec::new(), // assuming elements is a JSON array, handle deserialization properly
        };
        let rich_text: Vec<u8> = from_slice(&row.get::<Vec<u8>, _>("elements")).unwrap_or_default();
        let rich_text = RichText::decode(Bytes::from(rich_text)).unwrap();
        crate::servlet::olpush::msg::decoder::parse_elements(&bot, &mut record, rich_text.elems).await;
        Ok(record)
    }

}