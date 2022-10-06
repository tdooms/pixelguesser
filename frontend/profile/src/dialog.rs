use std::rc::Rc;

use cobul::{Box, Modal};
use yew::*;
use ywt::{callback, spawn};

use api::Result;
use shared::{use_auth, use_toast, UseAuthHandle};

use crate::{Additional, Credentials, Page};

async fn authenticate(cred: Rc<api::Credentials>, page: Page, auth: UseAuthHandle) -> Result<()> {
    match page {
        Page::Login => auth.login(cred).await,
        Page::Signup => auth.signup(cred).await,
        _ => unreachable!(),
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub page: Page,
    pub change: Callback<Page>,
}

#[function_component(Dialog)]
pub fn dialog(Props { change, page }: &Props) -> Html {
    let auth = use_auth();
    let toast = use_toast();

    let submit = callback!(page, auth, toast; move |credentials: Rc<api::Credentials>| {
        spawn!(page, credentials, auth, toast; async move {
            toast.maybe(authenticate(credentials, page, auth).await);
        });
    });

    let forgot = change.reform(|_| Page::Forgot);
    let signup = change.reform(|_| Page::Signup);
    let login = change.reform(|_| Page::Login);

    let inner = match page {
        Page::Login => {
            html! {
                <>
                <Credentials {submit} info="Log in to Pixelguesser to continue." />
                <div class="my-2"> <a onclick={forgot}> {"Forgot your password?"} </a> </div>
                <hr class="mb-2"/>
                <div class="has-text-centered">
                <span> {"Don't have an account? "} </span> <a onclick={signup}> {"Sign up"} </a>
                </div>
                </>
            }
        }
        Page::Signup => {
            html! {
                <>
                <Credentials {submit} info="Sign up to Pixelguesser to continue." />
                <hr class="mb-2"/>
                <div class="has-text-centered">
                <span> {"Already have an account? "} </span> <a onclick={login}> {"Log in"} </a>
                </div>
                </>
            }
        }
        Page::Forgot => {
            html! { <Box> {"that sucks :("} </Box> }
        }
        Page::Additional => {
            html! { <Additional /> }
        }
    };

    html! {
        <Modal active=true width=400>
            <Box> {inner} </Box>
        </Modal>
    }
}
