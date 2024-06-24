mod qqsecurity;
mod config;
mod args;
mod login;
mod backend;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use anyhow::Error;
use clap::Parser;
use ntrim_core::await_response;
use ntrim_core::bot::{Bot};
use ntrim_core::events::wtlogin_event::WtloginResponse;
use ntrim_tools::sigint;
use crate::args::{Args, LoginMode};
use crate::backend::{onebot, UID_UIN_MAP};
use crate::login::session::token_login;

const WELCOME: &str = r#"
  _   _ _____ ____  ___ __  __
 | \ | |_   _|  _ \|_ _|  \/  |
 |  \| | | | | |_) || || |\/| |
 | |\  | | | |  _ < | || |  | |
 |_| \_| |_| |_| \_\___|_|  |_|
 Welcome to ntrim!"#;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", args.log_level);
    }
    pretty_env_logger::init();
    sigint::init_sigint();
    info!("{}", WELCOME);

    let config = if let Some(path) = args.config_path {
        config::parse_local_config(std::path::PathBuf::from(path))
            .expect("Configuration file parsing failure")
    } else {
        let current_path = std::env::current_dir().unwrap();
        debug!("Current path: {:?}", current_path);
        config::parse_local_config(current_path.join("config.toml"))
            .expect("Configuration file parsing failure")
    };

    #[cfg(feature = "sql")]
    if config.sql.enable {
        ntrim_core::initialize_pool(&config.sql.address).await;
        ntrim_core::ensure_table_exists().await.expect("Failed to ensure table exists");
    }

    let ((bot, mut result), immediate_refresh) = match args.login_mode {
        LoginMode::Password { .. } => {
            panic!("Password login is not supported yet")
        }
        LoginMode::Session { session_path, immediate_refresh } => {
            (token_login(session_path, &config).await, immediate_refresh)
        }
    };

    loop {
        if result.is_closed() { return; }
        match result.recv().await.unwrap() {
            WtloginResponse::Success() => {
                break;
            }
            WtloginResponse::Fail(e) => {
                error!("Login failed: {}", e);
                return;
            }
            WtloginResponse::RefreshSigSuccess => panic!("RefreshSigSuccess is not supported yet") // 首次进入程序不该有这个分支
        };
    }

    // Here we can start the backend because the bot is online
    if immediate_refresh.map_or_else(|| false, |v| v) {
        if ntrim_core::refresh_session::refresh_sig(&bot).await {
            bot.client.set_lost().await;
        }
    }

    #[cfg(feature = "sql")]
    if config.sql.enable && std::env::var("IMM_REFRESH_CACHE").map_or(true, |v| v == "1") {
        info!("数据库支持已开启，开始刷新群列表/群成员列表/好友列表！");
        let start = std::time::Instant::now();
        let group_list = Bot::get_troop_list(&bot, true)
            .await.expect("Failed to get group list");
        info!("刷新群列表成功，共{}个群聊, 耗时: {:?}", group_list.len(), start.elapsed());
        for group_info in group_list {
            let start = std::time::Instant::now();
            match Bot::get_troop_member_list(&bot, group_info.code, group_info.owner_uin).await {
                Ok(list) => {
                    info!("刷新群成员列表成功，群号: {}, 共{}个成员, 耗时: {:?}", group_info.code, list.len(), start.elapsed());
                }
                Err(e) => {
                    warn!("Failed to get group member list for {}: {}", group_info.code, e)
                }
            }
        }

        let start = std::time::Instant::now();
        let friend_list = Bot::get_friend_list(&bot, true)
            .await.expect("Failed to get friend list");
        info!("刷新好友列表成功，共{}个好友, 耗时: {:?}", friend_list.friends.len(), start.elapsed());
    }

    let cache_mode = std::env::var("UID_CACHE_MODE").map_or("REVALIDATE".to_string(), |v| v);
    if cache_mode == "FULL" {
        let friend_list = Bot::get_friend_list(&bot, false)
            .await.expect("Failed to get friend list");
        for friend_info in friend_list.friends {
            UID_UIN_MAP.insert(friend_info.uin, friend_info.uid);
        }
        info!("好友列表缓存已刷新，共{}个好友，预估占用: {}kb", UID_UIN_MAP.len(), (40 * UID_UIN_MAP.len() * 56) as f64 / 1024.0);
    } else if cache_mode != "REVALIDATE" && cache_mode != "NONE" {
        warn!("Unknown UID_CACHE_MODE: {}, fallback to REVALIDATE", cache_mode);
        std::env::set_var("UID_CACHE_MODE", "REVALIDATE");
    }

    if let Ok(Some(profile)) = await_response!(tokio::time::Duration::from_secs(5), async {
        let rx = Bot::get_profile_detail(&bot, bot.unique_id).await;
        if let Some(rx) = rx {
            rx.await.map_err(|e| anyhow::Error::from(e))
        } else {
            Err(Error::msg("Unable to handle_get_qq_profile: tcp connection exception"))
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    }) {
        info!("欢迎你！{}，即此羡闲逸，怅然吟《式微》。", profile.nick_name);
    }

    info!("OneBot backend status: {}", cfg!(feature = "onebot"));
    info!("Kritor backend status: {}", cfg!(feature = "kritor"));

    if cfg!(feature = "onebot") {
        info!("Using OneBot backend, see https://github.com/botuniverse/onebot");
        onebot::launch(bot, config.onebot).await;
    } else if cfg!(feature = "kritor") {
        info!("Using Kritor backend, see https://github.com/KarinJS/kritor");

    } else {
        error!("No backend selected, please enable one of the backend features")
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}