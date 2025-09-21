use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FindAllRoomQuery {
    pub title: Option<String>,
}
