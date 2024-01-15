use actix_web::{self, App, HttpServer};
use tracing_subscriber;

pub static BIND_ADDRESS: &str = if cfg!(debug_assertions) {
    "127.0.0.1"
} else {
    "0.0.0.0"
};
pub static BIND_PORT: u16 = 9770;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("🎧🎧🎧 Starting Firestations API on port: {BIND_PORT}");

    HttpServer::new(move || {
        let app = App::new();

        return app;
    })
    .bind((BIND_ADDRESS, BIND_PORT))?
    .run()
    .await?;

    Ok(())
}
