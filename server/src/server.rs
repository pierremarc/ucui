use crate::config::{get_interface, get_port, get_static_dir};
use crate::state::UcuiState;
use axum::http::Method;
use axum::response::Redirect;
use axum::{routing::any, Router};
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use tower_http::trace::TraceLayer;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn start() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(tower_http::cors::Any);

    let router = Router::new()
        .route("/", any(|| async { Redirect::permanent("/play/") }))
        .route("/eco", any(crate::eco::lookup_eco))
        .route("/legals", any(crate::eco::legal_moves))
        .route("/engine", any(crate::play::handler))
        .route("/games", any(crate::monitor::handler))
        .fallback_service(ServeDir::new(get_static_dir()).append_index_html_on_directories(true))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(UcuiState::new());

    serve(router).unwrap();
}

fn serve(app: Router) -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    runtime.block_on(async {
        tokio::spawn(async move {
            let addr = SocketAddr::from((get_interface(), get_port()));
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            log::info!("listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, app).await.unwrap();
        })
        .await?;
        Ok(())
    })
}
