use std::time::SystemTime;

use axum::{Json, Router, routing};
use axum::extract::State;
use axum::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};

use uptime_api_db::entities;

use crate::controllers::{Claims, ReqSignIn, ResSignIn, verify_password};
use crate::controllers::error::Error::UserNotFound;
use crate::error::Error;
use crate::state::SharedAppState;

pub fn authentication_routes(shared_state: SharedAppState) -> Router {
    pub async fn test() -> &'static str {
        "Hello, World!"
    }

    #[axum::debug_handler]
    pub async fn login(
        State(app_state): State<SharedAppState>,
        Json(request_signin): Json<ReqSignIn>,
    ) -> Result<(StatusCode, Json<ResSignIn>), Error> {
        let Ok(user) = entities::users::find_by_email(request_signin.email.clone(), &app_state.db_pool).await else {
            return Err(Error::SIGNIN(UserNotFound));
        };

        if !verify_password(&request_signin.password, &user.password) {
            return Err(Error::SIGNIN(UserNotFound));
        }

        let claims = Claims {
            sub: user.email,
            role: "user".to_string(),
            exp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 4 * 60 * 60,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&app_state.jwt_secret.as_bytes()),
        ).unwrap();

        Ok((StatusCode::CREATED, Json(ResSignIn {
            token: token,
        })))
    }



    Router::new()
        .route("/login", routing::post(login))
        .with_state(shared_state)
}