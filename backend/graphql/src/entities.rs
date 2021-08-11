use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub name: String,
    pub description: String,
    pub author: String,
    pub created: DateTime<Utc>,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Round {
    answer: String,
}