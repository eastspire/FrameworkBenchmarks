use super::*;

#[route("/json")]
#[derive(Clone, Copy, Default)]
pub(crate) struct JsonRoute;

#[route("/plaintext")]
#[derive(Clone, Copy, Default)]
pub(crate) struct PlaintextRoute;

#[route("/db")]
#[derive(Clone, Copy, Default)]
pub(crate) struct DbRoute;

#[route("/query")]
#[derive(Clone, Copy, Default)]
pub(crate) struct QueryRoute;

#[route("/fortunes")]
#[derive(Clone, Copy, Default)]
pub(crate) struct FortunesRoute;

#[route("/upda")]
#[derive(Clone, Copy, Default)]
pub(crate) struct UpdateRoute;

#[route("/cached-quer")]
#[derive(Clone, Copy, Default)]
pub(crate) struct CachedQueryRoute;
