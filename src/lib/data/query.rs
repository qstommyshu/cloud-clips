use super::model;
use crate::data::{ DataError, DatabasePool };
use crate::ShortCode;
use sqlx::Row;

type Result<T> = std::result::Result<T, DataError>;

// TODO: explain
pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(
        sqlx
            ::query_as("SELECT * FROM clips WHERE shortcode = $1")
            .bind(shortcode)
            .fetch_one(pool).await? // will attempt to retrive one record from db, will get error if multiple are retrived
    )
}

// TODO: explain
pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx
        ::query(
            "INSERT INTO clips (
            clip_id, 
            shortcode, 
            content, 
            title, 
            posted, 
            expires, 
            password, 
            hits)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&model.clip_id)
        .bind(&model.shortcode)
        .bind(&model.content)
        .bind(&model.title)
        .bind(&model.posted)
        .bind(&model.expires)
        .bind(&model.password)
        .bind(0)
        .execute(pool).await?;
    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx
        ::query(
            "UPDATE clips SET
            content = $1,
            expires = $2,
            password = $3,
            title = $4,
            WHERE shortcode = $5"
        )
        .bind(&model.content)
        .bind(&model.expires)
        .bind(&model.password)
        .bind(&model.title)
        .bind(&model.shortcode)
        .execute(pool).await?;
    get_clip(model.shortcode, pool).await
}
