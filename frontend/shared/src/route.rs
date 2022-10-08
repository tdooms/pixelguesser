use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/manage/:code")]
    Manage { code: String },
    #[at("/host/:quiz_id")]
    Host { quiz_id: u64 },
    #[at("/create")]
    Create,
    #[at("/update/:quiz_id")]
    Update { quiz_id: u64 },
    #[at("/admin")]
    Admin,
    #[at("/sessions")]
    Sessions,
    #[at("/profile")]
    Profile,
    #[at("/library")]
    Library,
    #[at("/test")]
    Test,
    #[at("/")]
    Overview,
    #[not_found]
    #[at("/404")]
    NotFound,
}
