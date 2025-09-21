use crate::AppState;
use crate::constants;
use crate::dtos::global::SuccessResponse;
use crate::dtos::upvote::{CreateUpvoteRequest, CreateUpvoteResponse};
use crate::errors::Result;
use crate::extractors::UserId;
use crate::services;
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
    Json(request): Json<CreateUpvoteRequest>,
) -> Result<(StatusCode, CookieJar, SuccessResponse<CreateUpvoteResponse>)> {
    let created_upvote = services::upvote::create(&state.db, user_id, request).await?;

    Ok((
        StatusCode::CREATED,
        jar.add(Cookie::new(
            constants::cookies::USER_ID,
            user_id.to_string(),
        )),
        SuccessResponse::new(
            "Successfully Created new upvote",
            CreateUpvoteResponse::from(created_upvote),
        ),
    ))
}

pub async fn delete_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    UserId(user_id): UserId,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar)> {
    services::upvote::delete_by_id(&state.db, id, user_id).await?;

    Ok((
        StatusCode::NO_CONTENT,
        jar.add(Cookie::new(
            constants::cookies::USER_ID,
            user_id.to_string(),
        )),
    ))
}
