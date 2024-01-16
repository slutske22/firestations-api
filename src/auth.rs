use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// The claims contained in the JWT that should be part of every request Authorization header (common for keycloak JWTs)
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: u32,
    pub iat: u32,
    pub auth_time: u32,
    pub jti: String,
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub typ: String,
    pub azp: String,
    pub session_state: String,
    pub acr: String,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Vec<String>,
    pub scope: String,
    pub sid: String,
    pub email_verified: bool,
    pub preferred_username: String,
}

/// Function to extract the decoded JWT claims from the Auth header of a request
pub fn extract_jwt(req: HttpRequest) -> Result<TokenData<Claims>, anyhow::Error> {
    // Should unwrap correctly because if there is no Authorization header, code would
    // never have gotten this far, would have 401ed at the App()
    let authorization_header = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    let token = authorization_header
        .split_whitespace()
        .collect::<Vec<&str>>()[1];

    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    validation.set_audience(&["account"]);

    match decode::<Claims>(&token, &key, &validation) {
        Ok(claims) => Ok(claims),
        Err(e) => {
            HttpResponse::Unauthorized().finish();
            Err(anyhow::anyhow!("Could not decode JWT: {:?}", e))
        }
    }
}
