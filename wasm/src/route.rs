use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/manage/:session_id")]
    Manage { session_id: u64 },
    #[at("/quizzes/:quiz_id")]
    Host { quiz_id: i64 },
    #[at("/code")]
    Code,
    #[at("/")]
    Overview,
    #[not_found]
    #[at("/404")]
    NotFound,
}
