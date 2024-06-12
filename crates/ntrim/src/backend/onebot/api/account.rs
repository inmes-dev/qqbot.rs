use actix_web::{Responder, web};

mod get_account_info {
    use std::sync::Arc;
    use actix_web::{Error, Responder, web};
    use serde_derive::Deserialize;
    use tokio_stream::StreamExt;
    use ntrim_core::bot::Bot;
    use crate::backend::onebot::api::OnebotError;
    use crate::init_route;

    #[derive(Deserialize, Debug)]
    struct GetAccountInfoParams {
        debug: Option<bool>
    }

    async fn handle_get_account_info(data: &web::Data<Arc<Bot>>, params: GetAccountInfoParams) -> impl serde::Serialize {
        let user_id = data.unique_id;
        serde_json::json!({
            "user_id": user_id,
            "nickname": "unknown",
        })
    }

    init_route!("/get_account_info", GetAccountInfoParams, handle_get_account_info);
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(get_account_info::register)
    );
}