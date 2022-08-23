use crate::{Additional, Credentials, Page};
use cobul::{Box, Modal};
use shared::{use_auth, use_toast, UseAuthHandle, UseToastHandle};
use std::rc::Rc;
use yew::*;
use ywt::{callback, spawn};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub page: Page,
    pub onpage: Callback<Page>,
}

async fn authenticate(
    credentials: Rc<api::Credentials>,
    page: Page,
    toast: UseToastHandle,
    auth: UseAuthHandle,
) {
    let result = match page {
        Page::Login => auth.login(credentials).await,
        Page::Signup => auth.signup(credentials).await,
        _ => unreachable!(),
    };
    toast.maybe(result);
}

#[function_component(Popup)]
pub fn popup(Props { onpage, page }: &Props) -> Html {
    let auth = use_auth();
    let toast = use_toast();

    let onsubmit = callback!(page, auth, toast; move |credentials: Rc<api::Credentials>| {
        spawn!(page, credentials, auth, toast; authenticate(credentials, page, toast, auth));
    });

    let inner = match page {
        Page::Login => {
            html! { <Credentials onpage={onpage} onsubmit={onsubmit.clone()} signup=false /> }
        }
        Page::Forgot => {
            html! { <Box> {"that sucks :("} </Box> }
        }
        Page::Signup => {
            html! { <Credentials onpage={onpage} {onsubmit} signup=true /> }
        }
        Page::Additional => {
            html! { <Additional /> }
        }
    };

    html! {
        <Modal active=true width=400>
            {inner}
        </Modal>
    }
}
