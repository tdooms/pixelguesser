use cobul::props::{Color, ImageSize};
use cobul::*;
use yew::*;
use yew_router::prelude::*;

use shared::{Auth, Route};

#[function_component(Profile)]
pub fn profile() -> Html {
    let state = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let auth = use_context::<Auth>().unwrap();

    let onclick = {
        let cloned = state.clone();
        Callback::from(move |_| cloned.set(!*cloned))
    };
    let oncreate = Callback::from(move |_| navigator.push(Route::Create));
    let trigger = |picture| html! {<Image rounded=true src={picture} size={ImageSize::Is48x48}/>};

    match use_context::<Auth>().unwrap().user() {
        Ok(user) => html! {
            <>
                <Button color={Color::Primary} class="m-2" onclick={oncreate}>
                    <span>{"Create quiz"}</span>
                </Button>

                <Dropdown class="m-1 mr-2" trigger={trigger(user.picture.clone())} {onclick} active={*state} right=true>
                    <DropdownItem> {"Profile"} </DropdownItem>
                    <DropdownDivider/>
                    <DropdownItem onclick={auth.logout()}> {"Log out"} </DropdownItem>
                </Dropdown>
            </>
        },
        Err(false) => html! {
            <Buttons class="mx-2">
                <Button color={Color::Primary} onclick={auth.signup()}> <strong>{"Sign up"}</strong></Button>
                <Button light={true} onclick={auth.login()}> {"Log in"} </Button>
            </Buttons>
        },
        Err(true) => html! {},
    }
}
