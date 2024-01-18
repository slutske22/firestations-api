use actix_web::{
    get,
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::{self, ToSchema};

use crate::AppState;

/// Information about a firestation via the FEMA database, geocoded by Geocodio in 2019
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FireStation {
    /// The database id of the entry
    _id: String,
    /// The Fire Department ID
    #[serde(rename = "FDID")]
    fdid: String,
    /// The name of the fire department
    #[serde(rename = "Fire dept name")]
    name: String,
    /// Address of the fire department, line 1
    #[serde(rename = "HQ addr1")]
    addr1: String,
    /// Address of the fire department, line 2
    #[serde(rename = "HQ addr2")]
    addr2: String,
    /// City of the fire department address
    #[serde(rename = "HQ city")]
    city: String,
    /// State of the fire department address
    #[serde(rename = "HQ state")]
    state: String,
    /// Zip of the fire department address in `XXXXX-XXXX` format
    #[serde(rename = "HQ zip")]
    zip: String,
    /// Fire department phone number
    #[serde(rename = "HQ phone")]
    phone: String,
    /// Fire department fax number
    #[serde(rename = "HQ fax")]
    fax: String,
    /// County the fire department belongs to
    #[serde(rename = "County")]
    county: String,
    /// The type of fire department
    #[serde(rename = "Dept Type")]
    dept_type: String,
    /// The fire department organization type
    #[serde(rename = "Organization Type")]
    org_type: String,
    /// The website for the department, if any
    #[serde(rename = "Website")]
    website: String,
    /// The number of individual stations associated with the department
    #[serde(rename = "Number Of Stations")]
    number_of_stations: u32,
    /// The number of career, active firefighters
    #[serde(rename = "Active Firefighters - Career")]
    active_firefighters_career: u32,
    /// The number of volunteer active firefighers
    #[serde(rename = "Active Firefighters - Volunteer")]
    active_firefighters_volunteer: u32,
    /// The number of paid-per-call active firefighters
    #[serde(rename = "Active Firefighters - Paid per Call")]
    active_firefighters_ppc: u32,
    /// Number of non-firefighting civilian personnel
    #[serde(rename = "Non-Firefighting - Civilian")]
    non_firefighting_civilian: u32,
    /// Number of non-firefighting volunteer personnel
    #[serde(rename = "Non-Firefighting - Volunteer")]
    non_firefighting_volunteer: u32,
    /// Whether or not this department is the primary agency for emergency management
    /// Should be bool but DB is funky
    #[serde(rename = "Primary agency for emergency mgmt")]
    primary_agency_for_emergency_management: String,
    /// Latitude coordinate of department
    #[serde(rename = "Latitude")]
    lat: f64,
    /// Longitude coordinate of department
    #[serde(rename = "Longitude")]
    lng: f64,
    /// Accuracy score of the geocodio geocoding result
    #[serde(rename = "Accuracy Score")]
    accuracy_score: f32,
}

/// Query parameters that can be sent with GET request to filter results
#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct QueryParams {
    minlat: f64,
    maxlat: f64,
    minlng: f64,
    maxlng: f64,
}

/// Get all existing [RadioPlan] records.
#[utoipa::path(
  get,
  path = "/v1/stations",
  tag = "Plans",
  responses(
      (status = 200, description = "success", body = Vec<FireStation>),
      (status = 500, description = "internal server error", body = String),
  )
)]
#[get("/v1/stations")]
pub async fn list(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let query_string = Query::<QueryParams>::from_query(req.query_string()).unwrap();
    tracing::debug!("Query string is {:?}", query_string);

    HttpResponse::Ok().json({})
}
