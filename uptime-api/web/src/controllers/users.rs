use crate::{error::Error, state::SharedAppState};
use axum::{extract::Path, extract::State, http::StatusCode, routing, Json, Router};
use tracing::info;
use uptime_api_db::entities;
use uuid::Uuid;

pub fn user_routes(shared_state: SharedAppState) -> Router {
    #[axum::debug_handler]
    pub async fn create(
        State(app_state): State<SharedAppState>,
        Json(user): Json<(entities::users::UserChangeset)>,
    ) -> Result<(StatusCode, Json<entities::users::User>), Error> {
        match entities::users::find_by_email(user.email.clone(), &app_state.db_pool).await {
            Ok(_) => return Err(Error::Database(uptime_api_db::Error::RecordAlreadyExists)),
            Err(_) => (),
        };
        let user_result = entities::users::create(user, &app_state.db_pool).await;
        return match user_result {
            Ok(user) => {
                info!("responding with {:?}", user);
                Ok((StatusCode::CREATED, Json(user)))
            }
            Err(e) => Err(Error::Database(e)),
        };
    }

    #[axum::debug_handler]
    pub async fn read_all(
        State(app_state): State<SharedAppState>,
    ) -> Result<Json<Vec<entities::users::User>>, Error> {
        let users = entities::users::load_all(&app_state.db_pool).await?;

        info!("responding with {:?}", users);

        Ok(Json(users))
    }

    #[axum::debug_handler]
    pub async fn read_one(
        State(app_state): State<SharedAppState>,
        Path(id): Path<Uuid>,
    ) -> Result<() /* e.g. Json<entities::users::User> */, Error> {
        todo!("load resource via uptime_api_db's APIs, trace, and respond!")

        /* Example:
        let user = entities::users::load(id, &app_state.db_pool).await?;
        Ok(Json(user))
        */
    }

    #[axum::debug_handler]
    pub async fn update(
        State(app_state): State<SharedAppState>,
        Path(id): Path<Uuid>,
        Json(user): Json<() /* e.g. entities::users::UserChangeset */>,
    ) -> Result<() /* e.g. Json<entities::users::User> */, Error> {
        todo!("update resource via uptime_api_db's APIs, trace, and respond!")

        /* Example:
        let user = entities::users::update(id, user, &app_state.db_pool).await?;
        Ok(Json(user))
        */
    }

    #[axum::debug_handler]
    pub async fn delete(
        State(app_state): State<SharedAppState>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, Error> {
        todo!("delete resource via uptime_api_db's APIs, trace, and respond!")

        /* Example:
        entities::users::delete(id, &app_state.db_pool).await?;
        Ok(StatusCode::NO_CONTENT)
        */
    }

    Router::new()
        .route("/", routing::post(create))
        .route("/", routing::get(read_all))
        .route("/:id", routing::get(read_one))
        .route("/:id", routing::put(update))
        .route("/:id", routing::delete(delete))
        .with_state(shared_state)
}
