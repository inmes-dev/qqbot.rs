pub mod session;
pub mod bot;
pub mod events;
pub mod client;
pub mod commands;
pub mod refresh_session;
pub mod service;

/// Only current module can access the global module.
pub(crate) mod pb;
pub(crate) mod servlet;
pub(crate) mod reconnect;
pub(crate) mod jce;


pub use servlet::olpush::msg::Contact;
pub use servlet::olpush::msg::MessageRecord;

#[cfg(feature = "sql")]
pub mod db;
#[cfg(feature = "sql")]
pub use crate::db::initialize_pool;
#[cfg(feature = "sql")]
pub use crate::db::is_initialized;
#[cfg(feature = "sql")]
pub use crate::db::ensure_table_exists;