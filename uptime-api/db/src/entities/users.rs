use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
#[cfg(feature = "test-helpers")]
use fake::{faker::lorem::en::*, Dummy};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Postgres;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct User {
    // these are examples only
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Validate, Clone)]
pub struct UserChangeset {
    // these are examples only
    #[validate(length(min = 1))]
    pub email: String,
    pub password: String,
}

pub async fn load_all(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Vec<User>, crate::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, email,password,created_at, updated_at  FROM users"
    )
    .fetch_all(executor)
    .await?;
    Ok(users)
}

pub async fn load(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<User, crate::Error> {
    match sqlx::query_as!(
        User,
        "SELECT id, email,password,created_at, updated_at FROM users WHERE id = $1",
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(user) => Ok(user),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn find_by_email(
    email: String,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<User, crate::Error> {
    match sqlx::query_as!(
        User,
        "SELECT id, email,password,created_at, updated_at FROM users WHERE email = $1",
        email
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(user) => Ok(user),
        None => Err(crate::Error::NoRecordFound),
    }
}
pub async fn create(
    user: UserChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<User, crate::Error> {
    user.validate()?;

    let record = sqlx::query!(
        "INSERT INTO users (id,email,password,created_at, updated_at) VALUES ($1,$2,$3,$4,$5) RETURNING id, email, password, created_at, updated_at",
        Uuid::new_v4(),
        user.email,
        hash(&user.password, DEFAULT_COST).unwrap(),
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    )
    .fetch_one(executor)
    .await
    .map_err(crate::Error::DbError)?;

    Ok(User {
        id: record.id,
        email: user.email,
        password: record.password,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub async fn update(
    id: Uuid,
    user: UserChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<User, crate::Error> {
    user.validate()?;

    match sqlx::query!(
        "UPDATE users SET email = $1 WHERE id = $2 RETURNING id, email, password, created_at, updated_at",
        user.email,
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(record) => Ok(User {
            id: record.id,
            email: record.email,
            password: record.password,
            created_at: record.created_at,
            updated_at: record.updated_at
        }),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn delete(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<(), crate::Error> {
    match sqlx::query!("DELETE FROM users WHERE id = $1 RETURNING id", id)
        .fetch_optional(executor)
        .await
        .map_err(crate::Error::DbError)?
    {
        Some(_) => Ok(()),
        None => Err(crate::Error::NoRecordFound),
    }
}
