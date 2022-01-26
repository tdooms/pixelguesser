use cobul::props::{Color, ImageSize};
use cobul::*;
use yew::*;
use yew_agent::{use_bridge, UseBridgeHandle};
use yew_router::prelude::*;

use crate::agents::{UserAgent, UserInput};
use crate::shared::User;
use crate::Route;

#[function_component(Profile)]
pub fn profile() -> Html {
    let state = use_state(|| false);
    let bridge: UseBridgeHandle<UserAgent> = use_bridge(|_| ());

    let user = use_context::<User>();

    let onclick = {
        let cloned = state.clone();
        Callback::from(move |_| cloned.set(!*cloned))
    };

    let auth_callback = || {
        let cloned = bridge.clone();
        |msg: UserInput| Callback::from(move |_| cloned.send(msg))
    };

    let onsignup = auth_callback()(UserInput::Signup);
    let onlogin = auth_callback()(UserInput::Login);
    let onlogout = auth_callback()(UserInput::Logout);

    let history = use_history().unwrap();
    let oncreate = Callback::from(move |_| history.push(Route::Create));

    let anonymous = move || {
        html! {
            <Buttons class="mx-2">
                <Button color={Color::Primary} onclick={onsignup}> <strong>{"Sign up"}</strong></Button>
                <Button light={true} onclick={onlogin}> {"Log in"} </Button>
            </Buttons>
        }
    };

    log::info!("{}", *state);

    let known = move |user: &User| {
        let trigger = html! {
            <Image rounded=true src={user.picture.clone()} size={ImageSize::Is48x48}/>
        };

        html! {
            <>
                <Button color={Color::Primary} class="m-2" onclick={oncreate}>
                    <span>{"Create quiz"}</span>
                </Button>

                <Dropdown class="m-1 mr-2" {trigger} {onclick} active={*state} right=true>
                    <DropdownItem> {"Profile"} </DropdownItem>
                    <DropdownDivider/>
                    <DropdownItem onclick={onlogout}> {"Log out"} </DropdownItem>
                </Dropdown>
            </>
        }
    };

    match user {
        Some(user) => known(&user),
        None => anonymous(),
    }
}
