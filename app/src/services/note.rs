use crate::dtos::note::{CreateNoteRequest, UpdateNoteRequest};
use crate::entities::notes;
use crate::entities::sea_orm_active_enums::BoardEnum;
use crate::errors::{AppError, Result};
use crate::repositories;
use sea_orm::{ActiveEnum, ActiveValue, DatabaseConnection, IntoActiveModel};
use std::backtrace::Backtrace;
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    user_id: Uuid,
    request: CreateNoteRequest,
) -> Result<notes::Model> {
    let found_room = super::room::get_started_by_id(db, request.room_id).await?;

    let new_note = notes::ActiveModel {
        room_id: ActiveValue::Set(found_room.id),
        board: ActiveValue::Set(BoardEnum::try_from_value(&request.board)?),
        description: ActiveValue::Set(request.description),
        user_id: ActiveValue::Set(user_id),
        ..Default::default()
    };

    repositories::note::save(db, new_note).await
}

pub async fn get_today_by_id_and_user_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: Uuid,
) -> Result<notes::Model> {
    let found_note =
        repositories::note::get_active_by_id_and_active_room_and_user_id(db, id, user_id).await?;
    let Some((found_note, _)) = found_note else {
        return Err(AppError::NotFound(
            String::from("Note not found or expired"),
            Backtrace::capture(),
        ));
    };

    Ok(found_note)
}

pub async fn get_today_by_id(db: &DatabaseConnection, id: i32) -> Result<notes::Model> {
    let found_note = repositories::note::get_active_by_id_and_active_room(db, id).await?;
    let Some((found_note, _)) = found_note else {
        return Err(AppError::NotFound(
            String::from("Note not found or expired"),
            Backtrace::capture(),
        ));
    };

    Ok(found_note)
}

pub async fn update_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: Uuid,
    request: UpdateNoteRequest,
) -> Result<notes::Model> {
    let mut found_note = get_today_by_id_and_user_id(db, id, user_id)
        .await?
        .into_active_model();
    found_note.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());
    found_note.board = ActiveValue::Set(BoardEnum::try_from_value(&request.board)?);
    found_note.description = ActiveValue::Set(request.description);

    repositories::note::save(db, found_note).await
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32, user_id: Uuid) -> Result<()> {
    let mut found_note = get_today_by_id_and_user_id(db, id, user_id)
        .await?
        .into_active_model();
    found_note.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));
    repositories::note::save(db, found_note).await?;

    Ok(())
}
