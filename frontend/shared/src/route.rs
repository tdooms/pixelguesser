use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/manage/:code")]
    Manage { code: String },
    #[at("/host/:quiz_id")]
    Host { quiz_id: u32 },
    #[at("/create")]
    Create,
    #[at("/update/:quiz_id")]
    Update { quiz_id: u32 },
    #[at("/test")]
    Test,
    #[at("/admin/database")]
    Database,
    #[at("/")]
    Overview,
    #[not_found]
    #[at("/404")]
    NotFound,
}
