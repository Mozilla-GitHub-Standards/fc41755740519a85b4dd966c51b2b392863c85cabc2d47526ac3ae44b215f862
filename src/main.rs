//! Rustbox: A Rust based implementation of the Firefox Accounts Sync Storage service
//!
//! This app provides the FxA sync storage service (a.k.a pushbox). The idea being that
//! WebPush allows for small messages, but large, multipart messages can be problematic
//! for multiple reasons. Instead, FxA would use Pushbox as an imtermediary store, and
//! send the UA a link to data that it could fetch.
//!

#![feature(plugin, decl_macro, custom_derive, try_from)]
#![plugin(rocket_codegen)]
// #![cfg_attr(feature = "cargo-clippy", allow(new_ret_no_self))]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate futures;

extern crate mysql;
extern crate rand;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate rusoto_core;
extern crate rusoto_sqs;
extern crate serde;
extern crate slog_async;
extern crate slog_json;
extern crate slog_stdlog;
extern crate slog_term;

pub mod auth;
pub mod config;
pub mod db;
mod error;
mod logging;
pub mod server;
pub mod sqs;

use std::env;

use rocket::config::RocketConfig;

fn main() {
    // XXX: support ROCKET_LOG=off (coming in rocket 0.4)
    let (lk, lv) = env::vars()
        .find(|kv| kv.0.to_lowercase() == "rocket_log")
        .unwrap_or_else(|| ("rocket_log".to_owned(), "normal".to_owned()));
    let log_off = lv.to_lowercase() == "off";
    if log_off {
        // rocket 0.3 doesn't understand "off" yet, so remove it
        env::remove_var(lk);
    }

    // RocketConfig::init basically
    let rconfig = RocketConfig::read().unwrap_or_else(|_| {
        let path = env::current_dir()
            .unwrap()
            .join(&format!(".{}.{}", "default", "Rocket.toml"));
        RocketConfig::active_default(&path).unwrap()
    });

    // rocket::ignite basically
    let config = rconfig.active().clone();
    let rocket_serv = server::Server::start(rocket::custom(config, !log_off));
    rocket_serv.unwrap().launch();
}
