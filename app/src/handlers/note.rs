use crate::constants::cookies::USER_ID;
use crate::dtos::global::SuccessResponse;
use crate::dtos::note::{
    CreateNoteRequest, CreateNoteResponse, UpdateNoteRequest, UpdateNoteResponse,
};
use crate::errors::Result;
use crate::extractors::UserId;
use crate::{AppState, services};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use std::sync::Arc;

pub async fn create(
    State(state): State<Arc<AppState>>,
    UserId(user_id): UserId,
    jar: CookieJar,
    Json(request): Json<CreateNoteRequest>,
) -> Result<(StatusCode, CookieJar, SuccessResponse<CreateNoteResponse>)> {
    let new_note = services::note::create(&state.db, user_id, request).await?;

    Ok((
        StatusCode::CREATED,
        jar.add(Cookie::new(USER_ID, user_id.to_string())),
        SuccessResponse::new(
            "Successfully created note.",
            CreateNoteResponse::from(new_note),
        ),
    ))
}

pub async fn update_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    UserId(user_id): UserId,
    jar: CookieJar,
    Json(request): Json<UpdateNoteRequest>,
) -> Result<(StatusCode, CookieJar, SuccessResponse<UpdateNoteResponse>)> {
    let updated_note = services::note::update_by_id(&state.db, id, user_id, request).await?;

    Ok((
        StatusCode::OK,
        jar.add(Cookie::new(USER_ID, user_id.to_string())),
        SuccessResponse::new(
            "Successfully updated a note.",
            UpdateNoteResponse::from(updated_note),
        ),
    ))
}

pub async fn delete_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    UserId(user_id): UserId,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar)> {
    services::note::delete_by_id(&state.db, id, user_id).await?;

    Ok((
        StatusCode::NO_CONTENT,
        jar.add(Cookie::new(USER_ID, user_id.to_string())),
    ))
}
