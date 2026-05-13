use super::*;

impl ServerHook for JsonRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let json: Value = json!({
            KEY_MESSAGE: RESPONSEDATA_STR
        });
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_body(serde_json::to_vec(&json).unwrap_or_default())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        while stream.try_get_http_request().await.is_ok() {
            let data: Vec<u8> = ctx
                .get_mut_response()
                .set_body(serde_json::to_vec(&json).unwrap_or_default())
                .build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        Status::Continue
    }
}

impl ServerHook for PlaintextRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_body(RESPONSEDATA_BIN)
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        while stream.try_get_http_request().await.is_ok() {
            let data: Vec<u8> = ctx.get_mut_response().build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for DbRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let db_connection: &DbPoolConnection = get_db_connection();
        let query_row: QueryRow = random_world_row(db_connection).await;
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_body(serde_json::to_vec(&query_row).unwrap_or_default())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        while stream.try_get_http_request().await.is_ok() {
            let query_row: QueryRow = random_world_row(db_connection).await;
            let data: Vec<u8> = ctx
                .get_mut_response()
                .set_body(serde_json::to_vec(&query_row).unwrap_or_default())
                .build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for QueryRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let queries: Queries = ctx
            .get_request()
            .try_get_query(QUERY_DB_QUERY_KEY)
            .and_then(|queries| queries.parse::<Queries>().ok())
            .unwrap_or_default()
            .min(ROW_LIMIT as Queries)
            .max(1);
        let db_pool: &DbPoolConnection = get_db_connection();
        let data: Vec<QueryRow> = get_some_row_id(queries, db_pool).await;
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_body(serde_json::to_vec(&data).unwrap_or_default())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        while stream.try_get_http_request().await.is_ok() {
            let queries: Queries = ctx
                .get_request()
                .try_get_query(QUERY_DB_QUERY_KEY)
                .and_then(|queries| queries.parse::<Queries>().ok())
                .unwrap_or_default()
                .min(ROW_LIMIT as Queries)
                .max(1);
            let data: Vec<QueryRow> = get_some_row_id(queries, db_pool).await;
            let data: Vec<u8> = ctx
                .get_mut_response()
                .set_body(serde_json::to_vec(&data).unwrap_or_default())
                .build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for FortunesRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response().set_header(
            CONTENT_TYPE,
            ContentType::format_content_type_with_charset(TEXT_HTML, UTF8),
        );
        let all_rows: Vec<PgRow> = all_world_row().await;
        let mut fortunes_list: Vec<Fortunes> = all_rows
            .iter()
            .map(|row| {
                let id: i32 = row.get(KEY_ID);
                Fortunes::new(id, row.get(KEY_MESSAGE))
            })
            .collect();
        fortunes_list.push(Fortunes::new(
            0,
            "Additional fortune added at request time.".to_owned(),
        ));
        fortunes_list.sort_by(|it, next| it.message.cmp(&next.message));
        let res: String = FortunesTemplate::new(fortunes_list).to_string();
        let data: Vec<u8> = ctx.get_mut_response().set_body(&res).build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        while stream.try_get_http_request().await.is_ok() {
            let all_rows: Vec<PgRow> = all_world_row().await;
            let mut fortunes_list: Vec<Fortunes> = all_rows
                .iter()
                .map(|row| {
                    let id: i32 = row.get(KEY_ID);
                    Fortunes::new(id, row.get(KEY_MESSAGE))
                })
                .collect();
            fortunes_list.push(Fortunes::new(
                0,
                "Additional fortune added at request time.".to_owned(),
            ));
            fortunes_list.sort_by(|it, next| it.message.cmp(&next.message));
            let res: String = FortunesTemplate::new(fortunes_list).to_string();
            let data: Vec<u8> = ctx.get_mut_response().set_body(&res).build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for UpdateRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let queries: Queries = ctx
            .get_request()
            .try_get_query(UPDATE_DB_QUERY_KEY)
            .and_then(|queries| queries.parse::<Queries>().ok())
            .unwrap_or_default()
            .min(ROW_LIMIT as Queries)
            .max(1);
        let res: Vec<QueryRow> = update_world_rows(queries).await;
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_body(serde_json::to_vec(&res).unwrap_or_default())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        while stream.try_get_http_request().await.is_ok() {
            let queries: Queries = ctx
                .get_request()
                .try_get_query(UPDATE_DB_QUERY_KEY)
                .and_then(|queries| queries.parse::<Queries>().ok())
                .unwrap_or_default()
                .min(ROW_LIMIT as Queries)
                .max(1);
            let res: Vec<QueryRow> = update_world_rows(queries).await;
            let data: Vec<u8> = ctx
                .get_mut_response()
                .set_body(serde_json::to_vec(&res).unwrap_or_default())
                .build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for CachedQueryRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let count: Queries = ctx
            .get_request()
            .try_get_query(CACHE_QUERY_KEY)
            .and_then(|queries| queries.parse::<Queries>().ok())
            .unwrap_or_default()
            .min(ROW_LIMIT as Queries)
            .max(1);
        let res: Vec<&QueryRow> = CACHE.iter().take(count as usize).collect();
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_body(serde_json::to_vec(&res).unwrap_or_default())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        while stream.try_get_http_request().await.is_ok() {
            let count: Queries = ctx
                .get_request()
                .try_get_query(CACHE_QUERY_KEY)
                .and_then(|queries| queries.parse::<Queries>().ok())
                .unwrap_or_default()
                .min(ROW_LIMIT as Queries)
                .max(1);
            let res: Vec<&QueryRow> = CACHE.iter().take(count as usize).collect();
            let data: Vec<u8> = ctx
                .get_mut_response()
                .set_body(serde_json::to_vec(&res).unwrap_or_default())
                .build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}
