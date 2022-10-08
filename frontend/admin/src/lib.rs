mod sessions;

pub use sessions::*;

use cobul::*;
use shared::callback;
use shared::Route;
use yew::*;
use yew_router::prelude::use_navigator;

#[function_component(Admin)]
pub fn admin() -> Html {
    let navigator = use_navigator().unwrap();

    let sessions = callback!(navigator; move |_| navigator.push(&Route::Sessions));

    html! {
        <Buttons>
        <simple::Button text={"Sessions"} click={sessions} />
        </Buttons>
    }
}
