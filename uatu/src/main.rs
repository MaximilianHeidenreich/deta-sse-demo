use axum::{
    extract::{Path, State},
    http::{Method, Response, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Router,
};
use futures_util::stream::Stream;
use std::{env, sync::Arc};
use tokio::sync::broadcast;
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    StreamExt as _,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Default, Clone)]
struct Data {
    id: String,
    data: String,
}

impl Data {
    fn new(id: String, data: String) -> Self {
        Self { id, data }
    }
}

#[derive(Debug)]
struct AppState {
    tx: broadcast::Sender<Data>,
}

impl Default for AppState {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(1024 * 10);
        Self { tx }
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::HEAD])
        .allow_origin(Any);

    let adr = format!("0.0.0.0:{}", env::var("PORT").unwrap_or("8080".into()));
    let app = Router::new()
        .route("/", get(root))
        .route("/sse/:id", get(sse_handler))
        .route("/push/:id", post(push))
        .layer(cors)
        .with_state(Arc::new(AppState::default()));

    println!("[INFO ] serving on {adr:?}");

    axum::Server::bind(&adr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    r#"
POST /push/<id>
HEAD /sse/<id>
"#
}

async fn push(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    body: String,
) -> Response<String> {
    println!("[INFO ] new push: {id}:{body}");
    let mut resp = Response::default();

    match state.tx.send(Data::new(id, body)) {
        Ok(n) => {
            if n != state.tx.receiver_count() {
                println!("[INFO ] pushed to: {n} receivers");
            }
            *resp.body_mut() = "ok".into();
            *resp.status_mut() = StatusCode::OK;
        }

        Err(err) => {
            *resp.body_mut() = format!("internal error: {err}");
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    resp
}

async fn sse_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    println!("[INFO ] new subscriber: {id}");

    Sse::new(
        BroadcastStream::new(state.tx.subscribe())
            .filter(move |data| match data {
                Ok(Data { id: push_id, .. }) => *push_id == id,
                Err(err) => {
                    println!("{err}");
                    true
                }
            })
            .map(|data| data.map(|data| Event::default().data(data.data))),
    )
    .keep_alive(KeepAlive::default())
}
