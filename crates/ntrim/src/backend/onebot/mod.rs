mod http;
mod api;

use std::sync::Arc;
use ntrim_core::bot::Bot;
use crate::config::OneBot;


pub async fn launch(bot: Arc<Bot>, onebot: OneBot) {
    if onebot.http.enable {
        http::start(bot.clone(), onebot.http.host, onebot.http.port)
            .await.unwrap();
    }
}