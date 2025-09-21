use crate::entities::prelude::Upvotes;
use crate::entities::upvotes;
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
    let now = OffsetDateTime::now_utc();
    let today = now.date();

    let start_date = today.with_time(Time::MIDNIGHT);
    let end_date = today.with_time(Time::MAX);

    let found_upvote = Upvotes::find_by_id(id)
        .filter(upvotes::Column::UserId.eq(user_id))
        .filter(upvotes::Column::CreatedAt.between(start_date, end_date))
        .one(connection)
        .await?;
    Ok(found_upvote)
}

pub async fn delete(connection: &impl ConnectionTrait, model: upvotes::Model) -> Result<()> {
    model.delete(connection).await?;
    Ok(())
}
