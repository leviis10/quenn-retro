use crate::entities::rooms;
use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateRoomResponse {
    pub id: i32,

    pub title: String,

    pub class_level: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<rooms::Model> for UpdateRoomResponse {
    fn from(model: rooms::Model) -> Self {
        UpdateRoomResponse {
            id: model.id,
            title: model.title,
            class_level: model.class_level,
            cover: model.cover,
            description: model.description,
        }
    }
}
