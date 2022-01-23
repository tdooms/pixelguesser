use cobul::props::{Color, ImageSize};
use cobul::*;
use yew::*;
use yew_agent::{use_bridge, UseBridgeHandle};
use yew_router::prelude::*;

use crate::agents::{UserAgent, UserInput};
use crate::shared::{User, UserData};
use crate::Route;

#[derive(Default)]
struct State {
    user: User,
    expanded: bool,
}

#[function_component(Authentication)]
pub fn authentication() -> Html {
    let state = use_state(|| State::default());

    let bridge: UseBridgeHandle<UserAgent> = {
        let cloned = state.clone();
        use_bridge(move |user| cloned.set(State { user, expanded: false }))
    };

    let onclick = {
        let cloned = state.clone();
        Callback::from(move |_| {
            cloned.set(State { user: cloned.user.clone(), expanded: !cloned.expanded })
        })
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

    let State { user, expanded } = &*state;

    let anonymous = move || {
        html! {
            <Buttons extra="mx-2">
                <Button color={Color::Primary} onclick={onsignup}> <strong>{"Sign up"}</strong></Button>
                <Button light={true} onclick={onlogin}> {"Log in"} </Button>
            </Buttons>
        }
    };

    let known = move |user: &UserData| {
        html! {
            <>
                <Button color={Color::Primary} extra="m-2" onclick={oncreate}>
                    <span>{"Create quiz"}</span>
                </Button>
                <div class={classes!("dropdown", "is-right", "m-1", "mr-2", "is-clickable", expanded.then(|| "is-active"))}>
                    <div class="dropdown-trigger" {onclick}>
                        <Image rounded=true src={user.picture.clone()} size={ImageSize::Is48x48}/>
                    </div>
                    <div class="dropdown-menu" id="dropdown-menu" role="menu">
                        <div class="dropdown-content">
                            <a class="dropdown-item">{"Profile"}</a>
                            <hr class="dropdown-divider"/>
                            <a class="dropdown-item">{"Log out"}</a>
                        </div>
                    </div>
                </div>
            </>
            // <Buttons extra="mx-2">
            //     <Button light={true} onclick={onlogout}> {"Log out"} </Button>
            // </Buttons>
        }
    };

    match user {
        User::Loading => html! {"loading"},
        User::User(user) => known(&user),
        User::Anonymous => anonymous(),
    }
}
