use crate::entities::notes;
use crate::entities::prelude::Notes;
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

pub async fn get_today_active_by_id_and_user_id(
    connection: &impl ConnectionTrait,
    id: i32,
    user_id: Uuid,
) -> Result<Option<notes::Model>> {
    let now = OffsetDateTime::now_utc();
    let today = now.date();

    let start_date = today.with_time(Time::MIDNIGHT);
    let end_date = today.with_time(Time::MAX);

    let result = Notes::find_by_id(id)
        .filter(notes::Column::DeletedAt.is_null())
        .filter(notes::Column::UserId.eq(user_id))
        .filter(notes::Column::CreatedAt.between(start_date, end_date))
        .one(connection)
        .await?;

    Ok(result)
}
