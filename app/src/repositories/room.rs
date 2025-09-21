use crate::dtos::room::FindAllRoomQuery;
use crate::entities::prelude::Rooms;
use crate::entities::rooms;
use crate::errors::Result;
use crate::extractors::Pagination;
use sea_orm::sea_query::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, ItemsAndPagesNumber,
    PaginatorTrait, QueryFilter, TryIntoModel,
};
use time::{OffsetDateTime, Time};

pub async fn save(
    connection: &impl ConnectionTrait,
    model: rooms::ActiveModel,
) -> Result<rooms::Model> {
    let result = model.save(connection).await?.try_into_model()?;
    Ok(result)
}

pub async fn find_all_active_paginated(
    connection: &impl ConnectionTrait,
    pagination: &Pagination<FindAllRoomQuery>,
) -> Result<(ItemsAndPagesNumber, Vec<rooms::Model>)> {
    let mut query = Rooms::find().filter(rooms::Column::DeletedAt.is_null());
    if let Some(filters) = &pagination.query
        && let Some(title) = &filters.title
    {
        query = query.filter(Expr::col(rooms::Column::Title).ilike(format!("%{title}%")))
    }

    let paginator = query.paginate(connection, pagination.per_page);
    let total_results = paginator.num_items_and_pages().await?;
    let results = paginator
        .fetch_page(pagination.page.saturating_sub(1))
        .await?;

    Ok((total_results, results))
}

pub async fn get_active_by_id(
    connection: &impl ConnectionTrait,
    id: i32,
) -> Result<Option<rooms::Model>> {
    let result = Rooms::find_by_id(id)
        .filter(rooms::Column::DeletedAt.is_null())
        .one(connection)
        .await?;

    Ok(result)
}

pub async fn get_active_by_id_and_today(
    connection: &impl ConnectionTrait,
    id: i32,
) -> Result<Option<rooms::Model>> {
    let now = OffsetDateTime::now_utc();
    let today = now.date();

    let start_date = today.with_time(Time::MIDNIGHT);
    let end_date = today.with_time(Time::MAX);

    let result = Rooms::find_by_id(id)
        .filter(rooms::Column::DeletedAt.is_null())
        .filter(rooms::Column::CreatedAt.between(start_date, end_date))
        .one(connection)
        .await?;

    Ok(result)
}
