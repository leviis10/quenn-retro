use crate::errors::AppError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Deserialize)]
struct RawPagination {
    page: Option<u64>,
    per_page: Option<u64>,
}

pub struct Pagination<T> {
    pub page: u64,
    pub per_page: u64,
    pub query: Option<T>,
}

impl<T, S> FromRequestParts<S> for Pagination<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let query_str = parts.uri.query().unwrap_or("");

        let raw_pagination: RawPagination = serde_qs::from_str(query_str)?;
        let query: Option<T> = serde_qs::from_str(query_str).ok();

        let page = raw_pagination.page.unwrap_or(1);
        let per_page = raw_pagination.per_page.unwrap_or(10);
        Ok(Pagination {
            page,
            per_page,
            query,
        })
    }
}
