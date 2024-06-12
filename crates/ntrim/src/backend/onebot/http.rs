use std::sync::Arc;
use actix_web::{App, HttpServer, ResponseError, web};
use anyhow::Error;
use ntrim_core::bot::Bot;
use crate::backend::onebot::api;

pub(super) async fn start(bot: Arc<Bot>, host: String, port: u16) -> Result<(), Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bot.clone()))
            .configure(api::account::init_routes)
    })
        .bind((host, port))?
        .run()
        .await
        .map_err(|e| Error::from(e))?;
    Ok(())
}

