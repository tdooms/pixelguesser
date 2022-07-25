use cobul::*;
use yew::*;
use yew_router::prelude::*;
use ywt::callback;

use shared::{Auth, Route};

#[function_component(ProfileDropdown)]
pub fn profile_dropdown() -> Html {
    let focus = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let auth = use_context::<Auth>().unwrap();

    let onfocus = callback!(focus; move |new| focus.set(new));
    let oncreate = callback!(move |_| navigator.push(Route::Create));

    let trigger = |picture| {
        html! {
            <Image rounded=true src={picture} size={ImageSize::Is48x48}/>
        }
    };

    match use_context::<Auth>().unwrap().user() {
        Ok(user) => html! {
            <>
                <Button color={Color::Primary} class="m-2" onclick={oncreate}>
                    <span>{"Create"}</span>
                </Button>

                <Dropdown class="m-1 mr-2" trigger={trigger(user.picture.clone())} {onfocus} active={*focus} right=true>
                    <DropdownItem> {"Profile"} </DropdownItem>
                    <DropdownItem> {"Library"} </DropdownItem>
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
