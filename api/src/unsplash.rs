use crate::{Result, UNSPLASH_KEY};
use derive_more::Display;
use std::fmt::Formatter;
use strum::EnumIter;

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct UnsplashUser {
    pub id: String,
    pub username: String,
    pub name: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub links: UserLinks,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PhotoLinks {
    #[serde(rename = "self")]
    pub this: String,

    pub html: String,
    pub download: String,
    pub download_location: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct UserLinks {
    #[serde(rename = "self")]
    pub this: String,

    pub html: String,
    // TODO: other fields
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Photo {
    pub id: String,

    pub created_at: String,
    pub updated_at: Option<String>,
    pub promoted_at: Option<String>,

    pub width: u32,
    pub height: u32,

    pub color: String,
    pub blur_hash: String,

    pub description: Option<String>,
    pub alt_description: Option<String>,

    pub urls: Urls,
    pub links: PhotoLinks,

    pub categories: Vec<String>,
    pub likes: u32,
    pub liked_by_user: bool,
    pub current_user_collections: Vec<String>,
    pub sponsorship: Option<String>,

    pub user: UnsplashUser,
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Hash, Eq, Display, EnumIter)]
pub enum OrderBy {
    #[default]
    #[display(fmt = "relevant")]
    Relevant,
    #[display(fmt = "latest")]
    Latest,
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Hash, Eq, Display, EnumIter)]
pub enum ContentFilter {
    #[default]
    #[display(fmt = "SFW")]
    SFW,
    #[display(fmt = "All")]
    All,
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum Color {
//     BlackAndWhite,
//     Black,
//     White,
//     Yellow,
//     Orange,
//     Red,
//     Purple,
//     Magenta,
//     Green,
//     Teal,
//     Blue,
// }

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Display, EnumIter)]
pub enum Orientation {
    #[display(fmt = "landscape")]
    Landscape,
    #[display(fmt = "portrait")]
    Portrait,
    #[display(fmt = "squarish")]
    Squarish,
    #[display(fmt = "all")]
    All,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct FilterBy {
    pub query: String,
    pub page: u64,
    pub per_page: u64,
    pub order_by: OrderBy,
    // pub collections: Vec<String>,
    pub content_filter: ContentFilter,
    // pub color: Option<Color>,
    pub orientation: Orientation,
}

impl std::fmt::Display for FilterBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("query={}&", self.query))?;
        f.write_fmt(format_args!("page={}&", self.page))?;
        f.write_fmt(format_args!("per_page={}&", self.per_page))?;
        f.write_fmt(format_args!("order_by={}&", self.order_by))?;

        match self.content_filter {
            ContentFilter::SFW => f.write_str("content_filter=low&")?,
            ContentFilter::All => f.write_str("content_filter=high&")?,
        }
        if self.orientation != Orientation::All {
            f.write_fmt(format_args!("orientation={}", self.orientation))?;
        }

        Ok(())
    }
}

impl Default for FilterBy {
    fn default() -> Self {
        FilterBy {
            query: "".to_string(),
            page: 1,
            per_page: 10,
            order_by: OrderBy::default(),
            // collections: vec![],
            content_filter: ContentFilter::default(),
            // color: None,
            orientation: Orientation::Landscape,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub results: Vec<Photo>,
    pub total: u64,
    pub total_pages: u64,
}

pub fn unsplash_link() -> String {
    format!("https://unsplash.com/?utm_source=pixelguesser&utm_medium=referral")
}
pub fn author_link(photo: &Photo) -> String {
    format!("{}?utm_source=pixelguesser&utm_medium=referral", photo.user.links.html)
}

pub async fn search_photos(filter: FilterBy) -> Result<(Vec<Photo>, Option<u64>)> {
    if filter.query.is_empty() {
        return Ok((vec![], None));
    }

    let key = UNSPLASH_KEY.to_owned();
    let base = "https://api.unsplash.com/";
    let url = format!("{base}/search/photos?{filter}");

    let response: Response = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Client-ID {}", key))
        .send()
        .await?
        .json()
        .await?;

    Ok((response.results, Some(response.total_pages)))
}

pub async fn download(url: String) {
    let key = UNSPLASH_KEY.to_owned();
    let _ = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Client-ID {}", key))
        .send()
        .await;
}

// pub async fn download(location: String, access_key: String) -> Result<()> {
//     Ok(())
// }
