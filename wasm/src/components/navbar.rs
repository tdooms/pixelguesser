use crate::route::Route;
use yew::prelude::*;
use yew_router::components::Link;

pub fn navbar() -> Html {
    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <Link<Route> classes=classes!("navbar-item") route=Route::Overview>
                    <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28"/>
                </Link<Route>>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbar">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="navbar" class="navbar-menu">
                <div class="navbar-start">
                    <div class="buttons">
                        <Link<Route> classes=classes!("button", "is-primary") route=Route::Code>
                            <span>{"Quizmaster"}</span>
                        </Link<Route>>
                    </div>
                </div>

                <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons">
                            <a class="button is-primary"><strong>{"Sign up"}</strong></a>
                            <a class="button is-light">{"Log in"}</a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
