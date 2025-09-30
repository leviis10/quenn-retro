use crate::entities::prelude::{Notes, Rooms};
use crate::entities::{notes, rooms};
use crate::errors::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TryIntoModel,
};
use time::{OffsetDateTime, Time};
use uuid::Uuid;

pub async fn save(
    connection: &impl ConnectionTrait,
    model: notes::ActiveModel,
) -> Result<notes::Model> {
    let result = model.save(connection).await?.try_into_model()?;

    Ok(result)
}

pub async fn get_active_by_id_and_active_room_and_user_id(
    connection: &impl ConnectionTrait,
    id: i32,
    user_id: Uuid,
) -> Result<Option<(notes::Model, Option<rooms::Model>)>> {
    let today = OffsetDateTime::now_utc().date().with_time(Time::MIDNIGHT);

    let result = Notes::find_by_id(id)
        .find_also_related(Rooms)
        .filter(rooms::Column::CreatedAt.gte(today))
        .filter(rooms::Column::DeletedAt.is_null())
        .filter(notes::Column::DeletedAt.is_null())
        .filter(notes::Column::UserId.eq(user_id))
        .one(connection)
        .await?;

    Ok(result)
}

pub async fn get_active_by_id_and_active_room(
    connection: &impl ConnectionTrait,
    id: i32,
) -> Result<Option<(notes::Model, Option<rooms::Model>)>> {
    let today = OffsetDateTime::now_utc().date().with_time(Time::MIDNIGHT);

    let result = Notes::find_by_id(id)
        .find_also_related(Rooms)
        .filter(rooms::Column::CreatedAt.gte(today))
        .filter(rooms::Column::DeletedAt.is_null())
        .filter(notes::Column::DeletedAt.is_null())
        .one(connection)
        .await?;

    Ok(result)
}
