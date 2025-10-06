use crate::entities::notes;
use sea_orm::ActiveEnum;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteResponse {
    pub id: i32,

    pub room_id: i32,

    pub board: String,

    pub description: String,

    pub created_at: OffsetDateTime,

    pub updated_at: OffsetDateTime,

    pub deleted_at: Option<OffsetDateTime>,
}

impl From<notes::Model> for CreateNoteResponse {
    fn from(model: notes::Model) -> Self {
        CreateNoteResponse {
            id: model.id,
            room_id: model.room_id,
            board: model.board.to_value(),
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}
