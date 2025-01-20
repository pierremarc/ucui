use crate::config::{get_interface, get_port, get_static_dir};
use axum::{routing::any, Router};
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
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

    let router = Router::new()
        .route("/play", any(crate::play::handler))
        .fallback_service(ServeDir::new(get_static_dir()).append_index_html_on_directories(true))
        .layer(TraceLayer::new_for_http());

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
