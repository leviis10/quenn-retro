use crate::entities::rooms;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize)]
pub struct CreateRoomResponse {
    pub id: i32,

    pub title: String,

    pub class_level: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub created_at: OffsetDateTime,
}

impl From<rooms::Model> for CreateRoomResponse {
    fn from(model: rooms::Model) -> Self {
        CreateRoomResponse {
            id: model.id,
            title: model.title,
            class_level: model.class_level,
            cover: model.cover,
            description: model.description,
            created_at: model.created_at,
        }
    }
}
