use dotenv_codegen::dotenv;

pub const SELF_ENDPOINT: &str = "http://localhost:8900";
pub const HASURA_ENDPOINT: &str = "http://localhost:8901/v1/graphql";
pub const IMAGE_ENDPOINT: &str = "http://localhost:8902";
pub const SESSION_ENDPOINT: &str = "http://localhost:8903";
pub const WEBSOCKET_ENDPOINT: &str = "ws://localhost:8903/ws";
pub const AUTH_ENDPOINT: &str = "http://localhost:8904";

pub const IMAGE_PLACEHOLDER: &str = "https://unsplash.com/photos/gM8igOIP5MA";
pub const APPLICATION_NAME: &str = "pixelguesser";

pub const UNSPLASH_KEY: &str = dotenv!("UNSPLASH_KEY");
