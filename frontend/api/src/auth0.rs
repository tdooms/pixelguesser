#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub email: String,
    pub email_verified: bool,
    pub sub: String,
    pub token: String,
}
