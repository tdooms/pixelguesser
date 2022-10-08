use cobul::*;
use shared::callback;
use yew::*;
use yew_router::prelude::*;

use shared::{use_auth, Route};

use crate::Dialog;

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
    let change = callback!(page; move |new| page.set(Some(new)));

    let signup = change.reform(|_| Page::Signup);
    let login = change.reform(|_| Page::Login);

    let modal = page.map(|page| html! { <Dialog {page} {change} /> }).unwrap_or_default();

    html! {
        <>
        {modal}
        <Buttons class="mx-2">
            <Button color={Color::Info} click={signup}> <strong>{"Sign up"}</strong> </Button>
            <Button light={true} click={login}> {"Log in"} </Button>
        </Buttons>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct UserProps {
    create: Callback<()>,
    logout: Callback<()>,
}

#[function_component(UserMenu)]
pub fn user_menu(UserProps { create, logout }: &UserProps) -> Html {
    let focussed = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let focus = callback!(focussed; move |new| focussed.set(new));
    // let profile = callback!(navigator; move |_| navigator.push(Route::Profile));
    // let library = callback!(navigator; move |_| navigator.push(Route::Library));

    let trigger = html! {
        <div onclick={callback!(focussed; move |_| focussed.set(!*focussed))}>
        <Image rounded=true src={"yo"} size={ImageSize::Is48x48} />
        </div>
    };

    html! {
        <Buttons>
            <simple::Button color={Color::Info} class="m-2" click={create} text="Create" />

            // <Dropdown class="m-1 mr-2" {trigger} {focus} active={*focussed} right=true>
            //     <DropdownItem click={profile}> {"Profile"} </DropdownItem>
            //     <DropdownItem click={library}> {"Library"} </DropdownItem>
            //     <DropdownDivider/>
            //     <DropdownItem click={logout}> {"Log out"} </DropdownItem>
            // </Dropdown>
        </Buttons>
    }
}

#[function_component(Menu)]
pub fn menu() -> Html {
    let navigator = use_navigator().unwrap();
    let auth = use_auth();

    let create = callback!(navigator; move |_| navigator.push(&Route::Create));
    let logout = callback!(auth; move |_| auth.logout());

    match auth.token() {
        Some(_) => html! { <UserMenu {create} {logout} /> },
        None => html! { <AnonymousMenu /> },
    }
}
