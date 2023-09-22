mod app;
mod errors;

#[tokio::main]
async fn main() -> Result<(), errors::ApiError> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let app = app::get_app();

    let addr = std::env::var("KSOX_SERVER_WORKER_ADDRESS")?.parse()?;
    tracing::debug!("server starting at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown::listen().await;
        })
        .await?;
    Ok(())
}
