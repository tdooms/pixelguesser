use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/manage/:session_id")]
    Manage { session_id: String },
    #[at("/host/:quiz_id")]
    Host { quiz_id: u64 },
    #[at("/create")]
    Create,
    #[at("/test")]
    Test,
    #[at("/")]
    Overview,
    #[not_found]
    #[at("/404")]
    NotFound,
}
