use crate::constants::cookies::USER_ID;
use crate::errors::AppError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use uuid::Uuid;

pub struct UserId(pub Uuid);

impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await?;

        let Some(user_id) = jar.get(USER_ID) else {
            return Ok(UserId(Uuid::new_v4()));
        };

        let user_id = user_id.value().to_owned();
        let user_id = Uuid::parse_str(&user_id)?;
        Ok(UserId(user_id))
    }
}
