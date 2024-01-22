use crate::{openapi::ApiDoc, station::FireStation};
use actix_web::{self, web, App, HttpServer};
use logging::initialize_tracing;
use mongodb::{bson::doc, Client, Collection};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod logging;
mod openapi;
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
    initialize_tracing(Some("firestations_api_rust=debug"));

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
            .app_data(collection.clone())
            .service(station::list)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            );

        return app;
    })
    .bind((BIND_ADDRESS, BIND_PORT))?
    .run()
    .await?;

    Ok(())
}
