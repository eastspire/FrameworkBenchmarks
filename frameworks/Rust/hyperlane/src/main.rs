mod config;
mod db;
mod middleware;
mod route;
mod server;
mod utils;

use {config::*, db::*, server::*, utils::*};

use std::fmt;

use {
    futures::{executor::block_on, future::join_all},
    hyperlane::{
        tokio::{spawn, task::JoinHandle},
        *,
    },
    hyperlane_time::*,
    once_cell::sync::Lazy,
    rand::{RngExt, SeedableRng, rng, rngs::SmallRng},
    serde::*,
    serde_json::{Value, json},
    sqlx::{
        Pool, Postgres, Row,
        postgres::{PgPoolOptions, PgRow},
        query as db_query,
    },
};

#[tokio::main]
#[hyperlane(server: Server)]
async fn main() {
    init_db().await;
    server
        .server_config(init_server_config())
        .request_config(init_request_config());
    server.run().await.unwrap().wait().await;
}
