use router::{
    oak::{LogController, Logger},
    sqlite::SQLite3Connection,
};
use std::sync::Arc;

mod args;
mod common;
mod routes;

/// The LDHCP Server
struct LDHCPD {
    database: SQLite3Connection,
    updates: Logger,
}

router::router_main!(args::PARSER, routes::route);

impl router::Service for LDHCPD {
    type Error = std::convert::Infallible;
    type Options = args::Options;

    const RESOURCE: &str = "ldhcpd";

    fn new(
        _: args::Options,
        log_controller: &Arc<LogController>,
        database: SQLite3Connection,
    ) -> Result<Self, Self::Error> {
        let updates = log_controller.create_logger("updates");

        Ok(LDHCPD { database, updates })
    }
}
