// use crate::test_helpers::{BodyExt, DbTestContext, RouterExt};
// use axum::{
//     body::Body,
//     http::{self, Method},
// };
// use fake::{Fake, Faker};
// use googletest::prelude::*;
// use hyper::StatusCode;
// use uptime_api_db::{entities, transaction, Error};
// use uptime_api_macros::db_test;
// use serde_json::json;
// use std::collections::HashMap;
// use uuid::Uuid;
// use uptime_api_web::test_helpers::DbTestContext;
//
// use crate::common;
//
// #[db_test]
// async fn test_create_invalid(context: &DbTestContext) {
//     todo!("send invalid changeset, assert 422 response!");
//
//     /* Example:
//     let payload = json!(entities::users::UserChangeset {
//         description: String::from("")
//     });
//
//     let response = context
//         .app
//         .request("/users")
//         .method(Method::POST)
//         .body(payload.to_string())
//         .header(http::header::CONTENT_TYPE, "application/json")
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));
//     */
// }
//
// #[db_test]
// async fn test_create_success(context: &DbTestContext) {
//     todo!("send valid changeset, assert 201 response!");
//
//     /* Example:
//     let changeset: entities::users::UserChangeset = Faker.fake();
//     let payload = json!(changeset);
//
//     let response = context
//         .app
//         .request("/users")
//         .method(Method::POST)
//         .body(payload.to_string())
//         .header(http::header::CONTENT_TYPE, "application/json")
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::CREATED));
//
//     let users = users = entities::users::load_all(&app_state.db_pool);
//     assert_that!(users, len(eq(1)));
//     assert_that!(
//         users.first().unwrap().description,
//         eq(changeset.description)
//     );
//     */
// }
//
// #[db_test]
// async fn test_read_all(context: &DbTestContext) {
//     todo!("arrange DB, load all entities, assert all are returned!");
//
//     /* Example:
//     let changeset: entities::users::UserChangeset = Faker.fake();
//     entities::users::create(changeset.clone(), &context.db_pool)
//         .await
//         .unwrap();
//
//     let response = context
//         .app
//         .request("/users")
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::OK));
//
//     let users: Vec<entities::users::User> = response.into_body.into_json::<Vec<entities::users::User>>(response).await;
//     assert_that!(users, len(eq(1)));
//     assert_that!(
//         users.first().unwrap().description,
//         eq(changeset.description)
//     );
//     */
// }
//
// #[db_test]
// async fn test_read_one_nonexistent(context: &DbTestContext) {
//     todo!("read non-existent entity, assert 404 response!");
//
//     /* Example:
//     let response = context
//         .app
//         .request(&format!("/users/{}", Uuid::new_v4()))
//         .body(payload.to_string())
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
//     */
// }
//
// #[db_test]
// async fn test_read_one_success(context: &DbTestContext) {
//     todo!("arrange DB, load entity, assert it is returned!");
//
//     /* Example:
//     let user_changeset: entities::users::UserChangeset = Faker.fake();
//     let user = create_taskuser(user_changeset.clone(), &context.db_pool)
//         .await
//         .unwrap();
//     let user_id = user.id;
//
//     let response = context
//         .app
//         .request(&format!("/users/{}", user_id))
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::OK));
//
//     let user: entities::users::User = response.into_body().into_json::<entities::users::User>(response).await;
//     assert_that!(user.id, eq(user_id));
//     assert_that!(user.description, eq(user_changeset.description));
//     */
// }
//
// #[db_test]
// async fn test_update_invalid(context: &DbTestContext) {
//     todo!("arrange DB, send invalid changeset, assert 422 response and nothing changes in DB!");
//
//     /* Example:
//     let user_changeset: entities::users::UserChangesetChangeset = Faker.fake();
//     let user = create_task(user_changeset.clone(), &context.db_pool)
//         .await
//         .unwrap();
//
//     let payload = json!(entities::users::UserChangeset {
//         description: String::from("")
//     });
//
//     let response = context
//         .app
//         .request(&format!("/users/{}", user_id))
//         .method(Method::PUT)
//         .body(payload.to_string())
//         .header(http::header::CONTENT_TYPE, "application/json")
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::UNPROCESSABLE_ENTITY));
//
//     let user_after = load_user(user.id, &context.db_pool).await.unwrap();
//     assert_that!(user_after.description, eq(user.description));
//     */
// }
//
// #[db_test]
// async fn test_update_nonexistent(context: &DbTestContext) {
//     todo!("send valid changeset for non-existent ID, assert 404 response!");
//
//     /* Example:
//     let user_changeset: entities::users::UserChangeset = Faker.fake();
//     let payload = json!(user_changeset);
//
//     let response = context
//         .app
//         .request(&format!("/users/{}", Uuid::new_v4()))
//         .method(Method::PUT)
//         .body(payload.to_string())
//         .header(http::header::CONTENT_TYPE, "application/json")
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
//     */
// }
//
// #[db_test]
// async fn test_update_success(context: &DbTestContext) {
//     todo!("arrange DB, send valid changeset, assert 200 response and changes applied in DB!");
//
//     /* Example:
//     let user_changeset: entities::users::UserChangeset = Faker.fake();
//     let user = create_user(user_changeset.clone(), &context.db_pool)
//         .await
//         .unwrap();
//
//     let user_changeset: entities::users::UserChangeset = Faker.fake();
//     let payload = json!(user_changeset);
//
//     let response = context
//         .app
//         .request(&format!("/users/{}", user.id))
//         .method(Method::PUT)
//         .body(payload.to_string())
//         .header(http::header::CONTENT_TYPE, "application/json")
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::OK));
//
//     let user: entities::users::User = response.into_body.into_json::<Task>(response).await;
//     assert_that!(user.description, eq(user_changeset.description.clone()));
//
//     let user = load_user(user.id, &context.db_pool).await.unwrap();
//     assert_that!(user.description, eq(user_changeset.description));
//     */
// }
//
// #[db_test]
// async fn test_delete_nonexistent(context: &DbTestContext) {
//     todo!("delete non-existing ID, assert 404 response!");
//
//     /* Example:
//     let response = context
//         .app
//         .request(&format!("/users/{}", Uuid::new_v4()))
//         .method(Method::DELETE)
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::NOT_FOUND));
//     */
// }
//
// #[db_test]
// async fn test_delete_success(context: &DbTestContext) {
//     todo!("arrange DB, delete, assert 204 response and record deleted in dB!");
//
//     /* Example:
//     let user_changeset: entities::users::UserChangeset = Faker.fake();
//     let user = create_user(user_changeset.clone(), &context.db_pool)
//         .await
//         .unwrap();
//
//     let response = context
//         .app
//         .request(&format!("/users/{}", Uuid::new_v4()))
//         .method(Method::DELETE)
//         .send()
//         .await;
//
//     assert_that!(response.status(), eq(StatusCode::NO_CONTENT));
//
//     let result = load_user(user.id, &context.db_pool).await;
//     assert_that!(result, err(anything()));
//     */
// }
