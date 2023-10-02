use once_cell::sync::Lazy;

mod app;
mod errors;
mod jwt;

static BIND_ADDR: Lazy<String> =
    Lazy::new(|| std::env::var("KSOX_SERVER_API_BIND").expect("KSOX_SERVER_API_BIND must be set"));

#[tokio::main]
async fn main() -> Result<(), errors::ApiError> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let app = app::get_app();

    let addr = BIND_ADDR.parse()?;
    tracing::info!("🚀 server starting at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown::listen().await;
        })
        .await?;
    Ok(())
}
