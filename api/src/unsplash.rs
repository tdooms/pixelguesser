use crate::Result;

#[derive(serde::Deserialize, Debug)]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct User {
    pub id: String,

    pub username: String,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub this: String,

    pub html: String,
    pub download: String,
    pub download_location: String,
}

#[derive(serde::Deserialize, Debug)]
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
    pub links: Links,

    pub categories: Vec<String>,
    pub likes: u32,
    pub liked_by_user: bool,
    pub current_user_collections: Vec<String>,
    pub sponsorship: Option<String>,

    pub user: User,
}

#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub results: Vec<Photo>,
}

pub async fn search_photos(query: String, access_key: String) -> Result<Vec<Photo>> {
    let base = "https://api.unsplash.com/";
    let url = format!("{base}/search/photos?query={query}");

    let response: Response = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Client-ID {}", access_key))
        .send()
        .await?
        .json()
        .await?;

    Ok(response.results)
}

// pub async fn download(location: String, access_key: String) -> Result<()> {
//     Ok(())
// }
