mod get_account_info;
mod get_qq_profile;
mod set_qq_profile;
mod get_stranger_info;
mod get_friend_list;
mod get_group_info;
mod get_group_list;
mod get_group_member_list;

use actix_web::{web};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(get_account_info::register)
            .configure(get_qq_profile::register)
            .configure(get_stranger_info::register)
            .configure(get_friend_list::register)
            .configure(get_group_info::register)
            .configure(get_group_list::register)
            .configure(get_group_member_list::register)
            .configure(set_qq_profile::register)
    );
}