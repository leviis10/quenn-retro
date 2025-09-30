use crate::entities::prelude::{Notes, Rooms, Upvotes};
use crate::entities::{notes, rooms, upvotes};
use crate::errors::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait, QueryFilter,
    TryIntoModel,
};
use time::{OffsetDateTime, Time};
use uuid::Uuid;

pub async fn save(
    connection: &impl ConnectionTrait,
    model: upvotes::ActiveModel,
) -> Result<upvotes::Model> {
    let created = model.save(connection).await?.try_into_model()?;
    Ok(created)
}

pub async fn get_today_by_id(
    connection: &impl ConnectionTrait,
    id: i32,
    user_id: Uuid,
) -> Result<Option<upvotes::Model>> {
    let found_upvote_note = Upvotes::find_by_id(id)
        .filter(upvotes::Column::UserId.eq(user_id))
        .find_also_related(Notes)
        .filter(notes::Column::DeletedAt.is_null())
        .one(connection)
        .await?;
    if let Some((found_upvote, Some(found_note))) = found_upvote_note {
        let today = OffsetDateTime::now_utc().date().with_time(Time::MIDNIGHT);
        let found_note_room = Notes::find_by_id(found_note.id)
            .find_also_related(Rooms)
            .filter(rooms::Column::DeletedAt.is_null())
            .filter(rooms::Column::CreatedAt.gte(today))
            .one(connection)
            .await?;
        if let Some((_, Some(_))) = found_note_room {
            return Ok(Some(found_upvote));
        }
    }
    Ok(None)
}

pub async fn delete(connection: &impl ConnectionTrait, model: upvotes::Model) -> Result<()> {
    model.delete(connection).await?;
    Ok(())
}
