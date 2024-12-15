use crate::state::SharedAppState;
use axum::body::Body;
use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use tracing::{error, info};
use tracing::log::LevelFilter::Info;
use uptime_api_db::entities::users::find_by_email;
use crate::controllers::Claims;

#[tracing::instrument(skip_all, fields(rejection_reason = tracing::field::Empty))]
pub async fn validate_jwt(
    State(app_state): State<SharedAppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    info!("Validating JWT");
    let Some(auth_header) = req.headers().get(http::header::AUTHORIZATION) else {
        error!("No authorization header found in request");
        return Err(StatusCode::FORBIDDEN);
    };
    let auth_header_str = auth_header.to_str();
    let Ok(auth_header_value) = auth_header_str else {
        error!("Failed to parse authorization header");
        return Err(StatusCode::FORBIDDEN);
    };

    let mut header = auth_header_value.split_whitespace();
    let (bearer, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string(), &app_state.jwt_secret) {
        Ok(data) => data,
        Err(_) => {
            error!("Unable to decode token");
            return Err(StatusCode::UNAUTHORIZED);
        }

    };

    let Ok(current_user) = find_by_email(token_data.claims.sub, &app_state.db_pool).await else {
        error!("User not found");
        return Err(StatusCode::UNAUTHORIZED);
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}

pub fn decode_jwt(jwt_token: String, secret: &String) -> Result<TokenData<Claims>, StatusCode> {
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}