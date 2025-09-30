use crate::dtos::upvote::CreateUpvoteRequest;
use crate::entities::upvotes;
use crate::errors::{AppError, Result};
use crate::repositories;
use crate::services;
use sea_orm::{ActiveValue, DatabaseConnection};
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    user_id: Uuid,
    request: CreateUpvoteRequest,
) -> Result<upvotes::Model> {
    let found_note = services::note::get_today_by_id(db, request.note_id).await?;

    let new_upvote = upvotes::ActiveModel {
        note_id: ActiveValue::Set(found_note.id),
        user_id: ActiveValue::Set(user_id),
        ..Default::default()
    };

    repositories::upvote::save(db, new_upvote).await
}

async fn get_today_upvote_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: Uuid,
) -> Result<upvotes::Model> {
    let found_upvote = repositories::upvote::get_today_by_id(db, id, user_id).await?;
    let Some(upvote) = found_upvote else {
        return Err(AppError::NotFound(String::from(
            "Upvote data not found or expired",
        )));
    };
    Ok(upvote)
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32, user_id: Uuid) -> Result<()> {
    let found_upvote = get_today_upvote_by_id(db, id, user_id).await?;
    repositories::upvote::delete(db, found_upvote).await?;

    Ok(())
}
