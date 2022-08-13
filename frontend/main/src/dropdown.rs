use cobul::*;
use yew::*;
use yew_router::prelude::*;
use ywt::callback;

use shared::{use_auth, Route};

#[function_component(ProfileDropdown)]
pub fn profile_dropdown() -> Html {
    let active = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let auth = use_auth();

    let onfocus = callback!(active; move |new| active.set(new));
    let oncreate = callback!(move |_| navigator.push(Route::Create));

    let trigger = |src| html! { <Image rounded=true {src} size={ImageSize::Is48x48}/> };

    let logout = Callback::noop();
    let login = Callback::noop();
    let signup = Callback::noop();

    match (auth.user(), auth.loading()) {
        (Some(user), false) => html! {
            <>
                <Button color={Color::Primary} class="m-2" onclick={oncreate}>
                    <span>{"Create"}</span>
                </Button>

                <Dropdown class="m-1 mr-2" trigger={trigger(user.picture.clone())} {onfocus} active={*active} right=true>
                    <DropdownItem> {"Profile"} </DropdownItem>
                    <DropdownItem> {"Library"} </DropdownItem>
                    <DropdownDivider/>
                    <DropdownItem onclick={logout}> {"Log out"} </DropdownItem>
                </Dropdown>
            </>
        },
        (None, false) => html! {
            <Buttons class="mx-2">
                <Button color={Color::Primary} onclick={signup}> <strong>{"Sign up"}</strong></Button>
                <Button light={true} onclick={login}> {"Log in"} </Button>
            </Buttons>
        },
        _ => html! {},
    }
}
