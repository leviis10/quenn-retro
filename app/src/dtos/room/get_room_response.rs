use crate::entities::{notes, rooms, upvotes};
use sea_orm::ActiveEnum;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Upvote {
    pub id: i32,

    pub user_id: Uuid,

    pub created_at: OffsetDateTime,
}

impl From<upvotes::Model> for Upvote {
    fn from(model: upvotes::Model) -> Self {
        Upvote {
            id: model.id,
            user_id: model.user_id,
            created_at: model.created_at,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,

    pub board: String,

    pub description: String,

    pub created_at: OffsetDateTime,

    pub updated_at: OffsetDateTime,

    pub user_id: Uuid,

    pub upvotes: Vec<Upvote>,
}

impl From<(notes::Model, Vec<upvotes::Model>)> for Note {
    fn from((note, upvotes): (notes::Model, Vec<upvotes::Model>)) -> Self {
        let upvotes = upvotes.into_iter().map(Upvote::from).collect();

        Note {
            id: note.id,
            board: note.board.to_value(),
            description: note.description,
            created_at: note.created_at,
            updated_at: note.updated_at,
            user_id: note.user_id,
            upvotes,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRoomResponse {
    pub id: i32,

    pub title: String,

    pub class_level: String,

    pub cover: Option<String>,

    pub description: Option<String>,

    pub created_at: OffsetDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<Note>>,
}

impl From<rooms::Model> for GetRoomResponse {
    fn from(model: rooms::Model) -> Self {
        GetRoomResponse {
            id: model.id,
            title: model.title,
            class_level: model.class_level,
            cover: model.cover,
            description: model.description,
            created_at: model.created_at,
            notes: None,
        }
    }
}

impl From<(rooms::Model, Vec<(notes::Model, Vec<upvotes::Model>)>)> for GetRoomResponse {
    fn from((room, notes): (rooms::Model, Vec<(notes::Model, Vec<upvotes::Model>)>)) -> Self {
        let notes: Vec<Note> = notes.into_iter().map(Note::from).collect();

        GetRoomResponse {
            id: room.id,
            title: room.title,
            class_level: room.class_level,
            cover: room.cover,
            description: room.description,
            created_at: room.created_at,
            notes: Some(notes),
        }
    }
}
