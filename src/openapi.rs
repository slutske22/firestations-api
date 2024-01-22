use crate::station;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags((name = "FireStations API")),
    info(description = "FireStations DB"),
    paths(station::list),
    components(schemas(station::FireStation))
)]
pub struct ApiDoc;
