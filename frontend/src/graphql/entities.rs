use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use strum::EnumIter;
use web_sys::Url;

#[derive(Serialize_repr, Deserialize_repr, Display, EnumIter, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PointChoices {
    #[display(fmt = "none")]
    None = 0,
    #[display(fmt = "1")]
    One = 1,
    #[display(fmt = "2")]
    Two = 2,
    #[display(fmt = "3")]
    Three = 3,
    #[display(fmt = "4")]
    Four = 4,
    #[display(fmt = "5")]
    Five = 5,
}

impl Default for PointChoices {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Serialize_repr, Deserialize_repr, Display, EnumIter, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GuessChoices {
    #[display(fmt = "no limit")]
    Infinity = 0,
    #[display(fmt = "1")]
    One = 1,
    #[display(fmt = "2")]
    Two = 2,
    #[display(fmt = "3")]
    Three = 3,
}

impl Default for GuessChoices {
    fn default() -> Self {
        Self::Infinity
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Image {
    Url {
        url: String,
    },

    #[serde(skip)]
    Local {
        local: web_sys::File,
        url: String,
    },

    #[serde(skip)]
    None,
}

impl Default for Image {
    fn default() -> Self {
        Self::None
    }
}

impl Image {
    pub fn new(file: &web_sys::File) -> Self {
        Self::Local { local: file.clone(), url: Url::create_object_url_with_blob(file).unwrap() }
    }

    pub fn src(&self) -> Option<String> {
        match self {
            Image::Url { url } | Image::Local { url, .. } => Some(url.clone()),
            Image::None => None,
        }
    }
    pub fn name(&self) -> Option<String> {
        match self {
            Image::Url { url } => Some(url.clone()),
            Image::Local { local, .. } => Some(local.name()),
            Image::None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct RoundInfo {
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct RoundOptions {
    pub speed: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct DraftQuiz {
    pub name: String,
    pub description: String,
    pub creator: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DraftRound {
    #[serde(flatten)]
    pub info: RoundInfo,

    #[serde(flatten)]
    pub options: RoundOptions,

    #[serde(flatten)]
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Quiz {
    pub quiz_id: u64,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub created_at: DateTime<Utc>,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Round {
    pub round_id: u64,
    pub quiz_id: u64,
    pub index: u64,

    #[serde(flatten)]
    pub info: RoundInfo,

    #[serde(flatten)]
    pub options: RoundOptions,

    pub image_url: Option<String>,
}
