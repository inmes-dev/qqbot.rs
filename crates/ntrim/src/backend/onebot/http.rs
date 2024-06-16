use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use anyhow::Error;
use ntrim_core::bot::Bot;
use crate::backend::onebot::api::account::{ * };
use crate::backend::onebot::api::message::{ * };

pub(super) async fn start(bot: Arc<Bot>, host: String, port: u16) -> Result<(), Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bot.clone()))
            .configure(get_account_info::register)
            .configure(get_qq_profile::register)
            .configure(get_stranger_info::register)
            .configure(get_friend_list::register)
            .configure(get_group_info::register)
            .configure(get_group_list::register)
            .configure(get_group_member_list::register)
            .configure(get_group_member_info::register)
            .configure(set_qq_profile::register)
            .configure(send_like::register)
            .configure(send_private_msg::register)
    })
        .bind((host, port))?
        .run()
        .await
        .map_err(|e| Error::from(e))?;
    Ok(())
}

