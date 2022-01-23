use yew::prelude::*;
use yew_router::prelude::Link;

use super::Authentication;
use cobul::props::{Color, TextColor};
use cobul::*;

use crate::shared::Route;

#[function_component(MainNavbar)]
pub fn main_navbar() -> Html {
    let brand = html! {
        <Link<Route> classes={classes!("navbar-item")} to={Route::Overview}>
            <span><b><i>{"Pixelguesser"}</i></b></span>
        </Link<Route>>
    };

    let end = html! { <Authentication/> };

    html! {
        <Navbar brand={brand} burger=true end={end} onclick={Callback::noop()}/>
    }
}
