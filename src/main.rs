use crate::station::FireStation;
use actix_web::{self, web, App, HttpServer};
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use tracing_subscriber;

mod auth;
mod station;

pub static BIND_ADDRESS: &str = if cfg!(debug_assertions) {
    "127.0.0.1"
} else {
    "0.0.0.0"
};
pub static BIND_PORT: u16 = 9770;

/// Global HTTP server state shared across the worker thread pool.
#[derive(Debug, Clone)]
#[must_use]
pub struct AppState {
    client: Client,
    collection: Collection<FireStation>,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("🎧🎧🎧 Starting Firestations API on port: {BIND_PORT}");

    let cluster_uri = "mongodb+srv://slutske22:FSCluster@fscluster.tmsah.mongodb.net/?retryWrites=true&w=majority";
    let client = Client::with_uri_str(cluster_uri).await?;

    // Ping Atlas MongoDB to test connection
    client
        .database("FSCluster")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    tracing::info!("🍃 Successfully connection to MongoDB Atlas Server");

    let database = client.database("sample_mflix");
    let collection: Collection<FireStation> = database.collection("movies");

    let app_state = web::Data::new(AppState {
        client,
        collection: collection.clone(),
    });

    HttpServer::new(move || {
        let app = App::new()
            .app_data(app_state.clone())
            .app_data(collection.clone());

        return app;
    })
    .bind((BIND_ADDRESS, BIND_PORT))?
    .run()
    .await?;

    Ok(())
}
