mod send_private_msg;

use actix_web::{web};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            //.configure(crate::backend::onebot::api::account::get_account_info::register)
    );
}