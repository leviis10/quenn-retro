use crate::AppState;
use crate::dtos::global::{PaginationResponse, SuccessResponse};
use crate::dtos::room::{
    CreateRoomRequest, CreateRoomResponse, FindAllRoomQuery, GetRoomResponse, UpdateRoomRequest,
    UpdateRoomResponse,
};
use crate::errors::{AppError, Result};
use crate::extractors::Pagination;
use crate::services;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use std::sync::Arc;

pub async fn create(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateRoomRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateRoomResponse>)> {
    let Some(secret_key) = headers.get("Secret") else {
        return Err(AppError::Unauthorized(String::from("Missing secret key")));
    };
    if secret_key.to_str()? != "supersecret" {
        return Err(AppError::WrongSecret(String::from("Secret is not match")));
    }

    let new_room = services::room::create(&state.db, request).await?;

    let response = SuccessResponse::new(
        "Successfully created a new room",
        CreateRoomResponse::from(new_room),
    );
    Ok((StatusCode::CREATED, response))
}

pub async fn find_all(
    State(state): State<Arc<AppState>>,
    query: Pagination<FindAllRoomQuery>,
) -> Result<(StatusCode, SuccessResponse<Vec<GetRoomResponse>>)> {
    let (pagination, found_rooms) = services::room::find_all(&state.db, &query).await?;
    let data = found_rooms.into_iter().map(GetRoomResponse::from).collect();

    let response = SuccessResponse::new("Successfully find all rooms", data)
        .with_pagination(PaginationResponse::from((query, pagination)));
    Ok((StatusCode::OK, response))
}

pub async fn find_all_archive(
    State(state): State<Arc<AppState>>,
    query: Pagination<FindAllRoomQuery>,
) -> Result<(StatusCode, SuccessResponse<Vec<GetRoomResponse>>)> {
    let (pagination, found_rooms) = services::room::find_all_archive(&state.db, &query).await?;
    let data = found_rooms.into_iter().map(GetRoomResponse::from).collect();

    let response = SuccessResponse::new("Successfully find all rooms", data)
        .with_pagination(PaginationResponse::from((query, pagination)));
    Ok((StatusCode::OK, response))
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, SuccessResponse<GetRoomResponse>)> {
    let found_room = services::room::get_by_id(&state.db, id).await?;

    let response =
        SuccessResponse::new("Successfully get a room", GetRoomResponse::from(found_room));
    Ok((StatusCode::OK, response))
}

pub async fn update_by_id(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateRoomRequest>,
) -> Result<(StatusCode, SuccessResponse<UpdateRoomResponse>)> {
    let updated_room = services::room::update_by_id(&state.db, id, request).await?;

    let response = SuccessResponse::new(
        "Successfully update a room",
        UpdateRoomResponse::from(updated_room),
    );
    Ok((StatusCode::OK, response))
}

pub async fn delete_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    services::room::delete_by_id(&state.db, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
