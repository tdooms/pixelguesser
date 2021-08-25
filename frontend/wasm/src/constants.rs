use std::fmt::Display;

pub static IMAGE_ENDPOINT: &str = "localhost:8001";
pub static SESSION_ENDPOINT: &str = "localhost:8002";

pub static PLACEHOLDER: &str = "https://bulma.io/images/placeholders/480x320.png";

pub fn image_url(name: &impl Display) -> String {
    format!("http://{}/{}", IMAGE_ENDPOINT, name)
}

pub fn session_url() -> String {
    format!("ws://{}/ws", SESSION_ENDPOINT)
}
