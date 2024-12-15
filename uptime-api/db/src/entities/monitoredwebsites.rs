use chrono::{NaiveDateTime, Utc};
#[cfg(feature = "test-helpers")]
use fake::{faker::name::en::*, Dummy};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Postgres;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Monitoredwebsite {
    // these are examples only
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub url: String,
    pub check_interval: Option<i32>,
    pub latency_threshold: Option<i32>,
    pub consecutive_failures_threshold: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Deserialize, Validate, Clone)]
#[cfg_attr(feature = "test-helpers", derive(Serialize, Dummy))]
pub struct MonitoredwebsiteChangeset {
    // these are examples only
    #[validate(length(min = 1))]
    pub url: String,
    pub user_id: Uuid,
    pub check_interval: i32,
    pub latency_threshold: i32,
    pub consecutive_failures_threshold: i32,
}

pub async fn load_all(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Vec<Monitoredwebsite>, crate::Error> {
    let monitoredwebsites = sqlx::query_as!(Monitoredwebsite, "SELECT id, url, user_id, check_interval,latency_threshold,consecutive_failures_threshold,created_at,updated_at FROM monitored_websites")
        .fetch_all(executor)
        .await?;
    Ok(monitoredwebsites)
}

pub async fn load(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Monitoredwebsite, crate::Error> {
    match sqlx::query_as!(
        Monitoredwebsite,
        "SELECT id, url, user_id, check_interval,latency_threshold,consecutive_failures_threshold,created_at,updated_at FROM monitored_websites WHERE id = $1",
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(monitoredwebsite) => Ok(monitoredwebsite),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn create(
    monitoredwebsite: MonitoredwebsiteChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Monitoredwebsite, crate::Error> {
    monitoredwebsite.validate()?;

    let record = sqlx::query!(
        "INSERT INTO monitored_websites (url,user_id,check_interval,latency_threshold,consecutive_failures_threshold,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING id,url,user_id,check_interval,latency_threshold,consecutive_failures_threshold,created_at,updated_at",
        monitoredwebsite.url,
        monitoredwebsite.user_id,
        monitoredwebsite.check_interval,
        monitoredwebsite.latency_threshold,
        monitoredwebsite.consecutive_failures_threshold,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    )
    .fetch_one(executor)
    .await
    .map_err(crate::Error::DbError)?;

    Ok(Monitoredwebsite {
        id: record.id,
        url: record.url,
        user_id: record.user_id,
        check_interval: record.check_interval,
        latency_threshold: record.latency_threshold,
        consecutive_failures_threshold: record.consecutive_failures_threshold,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub async fn update(
    id: Uuid,
    monitoredwebsite: MonitoredwebsiteChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Monitoredwebsite, crate::Error> {
    monitoredwebsite.validate()?;

    match sqlx::query!(
        "UPDATE monitored_websites SET url = $1 WHERE id = $2 RETURNING id,url,user_id,check_interval,latency_threshold,consecutive_failures_threshold,created_at,updated_at",
        monitoredwebsite.url,
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(record) => Ok(Monitoredwebsite {
            id: record.id,
            url: record.url,
            user_id: record.user_id,
            check_interval: record.check_interval,
            latency_threshold: record.latency_threshold,
            consecutive_failures_threshold: record.consecutive_failures_threshold,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn delete(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<(), crate::Error> {
    match sqlx::query!("DELETE FROM monitored_websites WHERE id = $1 RETURNING id", id)
        .fetch_optional(executor)
        .await
        .map_err(crate::Error::DbError)?
    {
        Some(_) => Ok(()),
        None => Err(crate::Error::NoRecordFound),
    }
}
