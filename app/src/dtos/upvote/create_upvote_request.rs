use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpvoteRequest {
    pub note_id: i32,
}
