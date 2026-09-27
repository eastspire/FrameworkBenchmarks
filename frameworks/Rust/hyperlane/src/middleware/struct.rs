use super::*;

#[request_middleware]
#[derive(Clone, Copy, Default)]
pub(crate) struct RequestMiddleware;
