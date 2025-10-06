use crate::extractors::Pagination;
use axum::Json;
use axum::response::{IntoResponse, Response};
use sea_orm::ItemsAndPagesNumber;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationResponse {
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
    pub total_items: u64,
}

impl<T> From<(Pagination<T>, ItemsAndPagesNumber)> for PaginationResponse {
    fn from((query, pagination): (Pagination<T>, ItemsAndPagesNumber)) -> Self {
        PaginationResponse {
            page: query.page,
            per_page: query.per_page,
            total_pages: pagination.number_of_pages,
            total_items: pagination.number_of_items,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse<T: Serialize> {
    message: String,

    data: T,

    pagination: Option<PaginationResponse>,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(message: &str, data: T) -> Self {
        SuccessResponse {
            message: String::from(message),
            data,
            pagination: None,
        }
    }

    pub fn with_pagination(mut self, pagination: PaginationResponse) -> Self {
        self.pagination = Some(pagination);
        self
    }
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
