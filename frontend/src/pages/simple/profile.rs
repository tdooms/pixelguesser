use crate::{User, UserAgent};
use yew::*;
use yew_agent::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    let state = use_state(|| User::default());

    let bridge: UseBridgeHandle<UserAgent> = {
        let cloned = state.clone();
        use_bridge(move |user| cloned.set(user))
    };

    html! {}
}
