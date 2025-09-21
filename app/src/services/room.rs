use crate::dtos::room::{CreateRoomRequest, FindAllRoomQuery, UpdateRoomRequest};
use crate::entities::rooms;
use crate::errors::{AppError, Result};
use crate::extractors::Pagination;
use crate::repositories;
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel, ItemsAndPagesNumber};
use time::OffsetDateTime;

pub async fn create(db: &DatabaseConnection, request: CreateRoomRequest) -> Result<rooms::Model> {
    let new_room = rooms::ActiveModel {
        title: ActiveValue::Set(request.title),
        class_level: ActiveValue::Set(request.class_level),
        cover: ActiveValue::Set(request.cover),
        description: ActiveValue::Set(request.description),
        ..Default::default()
    };

    repositories::room::save(db, new_room).await
}

pub async fn find_all(
    db: &DatabaseConnection,
    query: &Pagination<FindAllRoomQuery>,
) -> Result<(ItemsAndPagesNumber, Vec<rooms::Model>)> {
    repositories::room::find_all_active_paginated(db, query).await
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<rooms::Model> {
    let found_room = repositories::room::get_active_by_id(db, id).await?;
    let Some(found_room) = found_room else {
        return Err(AppError::NotFound(String::from("Room not found.")));
    };
    Ok(found_room)
}

pub async fn get_today_by_id(db: &DatabaseConnection, id: i32) -> Result<rooms::Model> {
    let found_room = repositories::room::get_active_by_id_and_today(db, id).await?;
    let Some(found_room) = found_room else {
        return Err(AppError::NotFound(String::from(
            "Room not found or Expired.",
        )));
    };
    Ok(found_room)
}

pub async fn update_by_id(
    db: &DatabaseConnection,
    id: i32,
    request: UpdateRoomRequest,
) -> Result<rooms::Model> {
    let mut found_room = get_by_id(db, id).await?.into_active_model();
    found_room.title = ActiveValue::Set(request.title);
    found_room.class_level = ActiveValue::Set(request.class_level);
    found_room.cover = ActiveValue::Set(request.cover);
    found_room.description = ActiveValue::Set(request.description);
    found_room.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());

    repositories::room::save(db, found_room).await
}

pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
    let mut found_room = get_by_id(db, id).await?.into_active_model();
    found_room.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));

    repositories::room::save(db, found_room).await?;

    Ok(())
}
