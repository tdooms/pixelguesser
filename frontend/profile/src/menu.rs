use crate::Popup;
use cobul::*;
use shared::{use_auth, Route};
use yew::*;
use yew_router::prelude::*;
use ywt::callback;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Login,
    Signup,
    Forgot,
    Additional,
}

#[derive(Properties, PartialEq)]
pub struct AnonymousProps {}

#[function_component(AnonymousMenu)]
pub fn anonymous_menu(props: &AnonymousProps) -> Html {
    let page = use_state(|| None);
    let onpage = callback!(page; move |new| page.set(Some(new)));

    let onsignup = onpage.reform(|_| Page::Signup);
    let onlogin = onpage.reform(|_| Page::Login);

    let modal = page.map(|page| html! { <Popup {page} {onpage} /> }).unwrap_or_default();

    html! {
        <>
        {modal}
        <Buttons class="mx-2">
            <Button color={Color::Primary} onclick={onsignup}> <strong>{"Sign up"}</strong></Button>
            <Button light={true} onclick={onlogin}> {"Log in"} </Button>
        </Buttons>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct UserProps {
    oncreate: Callback<()>,
    onlogout: Callback<()>,
}

#[function_component(UserMenu)]
pub fn user_menu(props: &UserProps) -> Html {
    let focus = use_state(|| false);
    let onfocus = callback!(focus; move |new| focus.set(new));

    let trigger = |src| html! { <Image rounded=true {src} size={ImageSize::Is48x48}/> };

    html! {
        <Buttons>
            <Button color={Color::Primary} class="m-2" onclick={&props.oncreate}>
                <span>{"Create"}</span>
            </Button>

            <Dropdown class="m-1 mr-2" trigger={trigger(Some("yo"))} {onfocus} active={*focus} right=true>
                <DropdownItem> {"Profile"} </DropdownItem>
                <DropdownItem> {"Library"} </DropdownItem>
                <DropdownDivider/>
                <DropdownItem onclick={&props.onlogout}> {"Log out"} </DropdownItem>
            </Dropdown>
        </Buttons>
    }
}

#[function_component(Menu)]
pub fn menu() -> Html {
    let navigator = use_navigator().unwrap();
    let auth = use_auth();

    let oncreate = callback!(navigator; move |_| navigator.push(Route::Create));
    let onlogout = callback!(auth; move |_| auth.logout());

    match auth.token() {
        Some(_) => html! { <UserMenu {oncreate} {onlogout} /> },
        None => html! { <AnonymousMenu /> },
    }
}
