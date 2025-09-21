use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoomRequest {
    pub title: String,

    pub class_level: String,

    pub cover: Option<String>,

    pub description: Option<String>,
}
