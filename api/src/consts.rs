use dotenv_codegen::dotenv;

pub const SELF_ENDPOINT: &str = dotenv!("SELF_ENDPOINT");
pub const GRAPHQL_ENDPOINT: &str = dotenv!("GRAPHQL_ENDPOINT");
pub const UPLOAD_ENDPOINT: &str = dotenv!("UPLOAD_ENDPOINT");
pub const IMAGE_ENDPOINT: &str = dotenv!("IMAGE_ENDPOINT");
pub const SESSION_CREATE_ENDPOINT: &str = dotenv!("SESSION_CREATE_ENDPOINT");
pub const SESSION_WS_ENDPOINT: &str = dotenv!("SESSION_WS_ENDPOINT");
pub const AUTH_ENDPOINT: &str = dotenv!("AUTH_ENDPOINT");

pub const IMAGE_PLACEHOLDER: &str = dotenv!("IMAGE_PLACEHOLDER");

pub const UNSPLASH_KEY: &str = dotenv!("UNSPLASH_KEY");
