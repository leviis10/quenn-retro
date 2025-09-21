use crate::entities::upvotes;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpvoteResponse {
    pub id: i32,

    pub note_id: i32,

    pub user_id: Uuid,

    pub created_at: OffsetDateTime,
}

impl From<upvotes::Model> for CreateUpvoteResponse {
    fn from(model: upvotes::Model) -> Self {
        CreateUpvoteResponse {
            id: model.id,
            note_id: model.note_id,
            user_id: model.user_id,
            created_at: model.created_at,
        }
    }
}
