use crate::graphql::{RoundInfo, RoundOptions};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SerRound {
    pub quiz_id: u64,
    pub index: u64,

    #[serde(flatten)]
    pub info: RoundInfo,
    #[serde(flatten)]
    pub options: RoundOptions,
    pub image_url: Option<String>,
}