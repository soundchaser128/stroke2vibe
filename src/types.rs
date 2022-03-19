use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Funscript {
    pub actions: Vec<Action>,
    pub metadata: Option<Metadata>,
    pub range: Option<f64>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub at: i64,
    pub pos: f64,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ty: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub duration: Option<f64>,

    pub average_speed: Option<f64>,

    #[serde(with = "string_empty_as_none")]
    pub creator: Option<String>,

    #[serde(with = "string_empty_as_none")]
    pub description: Option<String>,

    #[serde(with = "string_empty_as_none")]
    pub license: Option<String>,

    #[serde(with = "string_empty_as_none")]
    pub notes: Option<String>,

    pub performers: Option<Vec<String>>,

    #[serde(with = "string_empty_as_none")]
    pub script_url: Option<String>,

    pub tags: Option<Vec<String>>,

    #[serde(with = "string_empty_as_none")]
    pub title: Option<String>,

    #[serde(rename = "type")]
    pub ty: Option<String>,

    #[serde(with = "string_empty_as_none")]
    pub video_url: Option<String>,
}
