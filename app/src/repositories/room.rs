use crate::dtos::room::FindAllRoomQuery;
use crate::entities::prelude::{Notes, Rooms, Upvotes};
use crate::entities::{notes, rooms, upvotes};
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
    let today_midnight = OffsetDateTime::now_utc().date().with_time(Time::MIDNIGHT);

    let mut query = Rooms::find()
        .filter(rooms::Column::DeletedAt.is_null())
        .filter(rooms::Column::CreatedAt.gte(today_midnight));
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

pub async fn find_all_archive_paginated(
    connection: &impl ConnectionTrait,
    pagination: &Pagination<FindAllRoomQuery>,
) -> Result<(ItemsAndPagesNumber, Vec<rooms::Model>)> {
    let today_midnight = OffsetDateTime::now_utc().date().with_time(Time::MIDNIGHT);

    let mut query = Rooms::find()
        .filter(rooms::Column::DeletedAt.is_null())
        .filter(rooms::Column::CreatedAt.lt(today_midnight));
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

pub async fn get_active_by_id_and_started(
    connection: &impl ConnectionTrait,
    id: i32,
) -> Result<Option<rooms::Model>> {
    let start_date = OffsetDateTime::now_utc().date().with_time(Time::MIDNIGHT);

    let result = Rooms::find_by_id(id)
        .filter(rooms::Column::DeletedAt.is_null())
        .filter(rooms::Column::CreatedAt.gte(start_date))
        .one(connection)
        .await?;

    Ok(result)
}

pub async fn get_active_by_id(
    connection: &impl ConnectionTrait,
    id: i32,
) -> Result<Option<(rooms::Model, Vec<(notes::Model, Vec<upvotes::Model>)>)>> {
    let found_rooms = Rooms::find_by_id(id)
        .find_with_related(Notes)
        .filter(notes::Column::DeletedAt.is_null())
        .filter(rooms::Column::DeletedAt.is_null())
        .all(connection)
        .await?;

    let mut result = Vec::new();
    for (room, notes) in found_rooms {
        let mut note_with_upvotes = Vec::new();
        for note in notes {
            let found_note = Notes::find_by_id(note.id)
                .find_with_related(Upvotes)
                .all(connection)
                .await?;
            for (note, upvotes) in found_note {
                note_with_upvotes.push((note, upvotes));
            }
        }
        result.push((room, note_with_upvotes));
    }

    let result = result.first().cloned();

    Ok(result)
}
