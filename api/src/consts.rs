use dotenv_codegen::dotenv;

pub const HASURA_ENDPOINT: &str = dotenv!("HASURA_ENDPOINT");
pub const SELF_ENDPOINT: &str = dotenv!("SELF_ENDPOINT");
pub const IMAGE_ENDPOINT: &str = dotenv!("IMAGE_ENDPOINT");
pub const SESSION_ENDPOINT: &str = dotenv!("SESSION_ENDPOINT");
pub const AUTH_ENDPOINT: &str = dotenv!("AUTH_ENDPOINT");
pub const WEBSOCKET_ENDPOINT: &str = dotenv!("WEBSOCKET_ENDPOINT");

pub const IMAGE_PLACEHOLDER: &str = dotenv!("IMAGE_PLACEHOLDER");
pub const UNSPLASH_KEY: &str = dotenv!("UNSPLASH_KEY");
pub const APPLICATION_NAME: &str = dotenv!("APPLICATION_NAME");
