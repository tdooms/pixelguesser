use cobul::props::{Color, ImageSize};
use cobul::*;
use yew::*;
use yew_agent::use_bridge;
use yew_router::prelude::*;

use agents::{Auth, AuthAgent, UserInput};
use shared::Route;

#[function_component(Profile)]
pub fn profile() -> Html {
    let state = use_state(|| false);
    let bridge = use_bridge::<AuthAgent, _>(|_| ());

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

    let trigger = |picture| html! {<Image rounded=true src={picture} size={ImageSize::Is48x48}/>};

    match use_context::<Auth>().unwrap() {
        Auth::User(user) => html! {
            <>
                <Button color={Color::Primary} class="m-2" onclick={oncreate}>
                    <span>{"Create quiz"}</span>
                </Button>

                <Dropdown class="m-1 mr-2" trigger={trigger(user.picture.clone())} {onclick} active={*state} right=true>
                    <DropdownItem> {"Profile"} </DropdownItem>
                    <DropdownDivider/>
                    <DropdownItem onclick={onlogout}> {"Log out"} </DropdownItem>
                </Dropdown>
            </>
        },
        Auth::Anonymous => html! {
            <Buttons class="mx-2">
                <Button color={Color::Primary} onclick={onsignup}> <strong>{"Sign up"}</strong></Button>
                <Button light={true} onclick={onlogin}> {"Log in"} </Button>
            </Buttons>
        },
        Auth::Loading => html! {},
    }
}
