use yew::prelude::*;
use yew_router::components::Link;
use pbs::prelude::*;
use pbs::properties::Color;

use crate::route::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub src: String,

    pub name: String,

    pub creator: String,

    pub description: String,

    #[prop_or_default]
    pub route: Option<Route>,
}

#[function_component(QuizCard)]
pub fn quiz_card(props: &Props) -> Html {
    let footer = match &props.route {
        Some(route) => html! {
            <Link<Route> classes={classes!("button", "is-success", "is-fullwidth", "has-square-top")} route={route.clone()}>
                <Icon icon={"fas fa-play"}/> <span>{"Play"}</span>
            </Link<Route>>
        },
        None => html! {
            <Button color={Color::Success} fullwidth=true extra={"has-square-top has-no-pointer"}>
                <Icon icon={"fas fa-play"}/> <span>{"Play"}</span>
            </Button>
        },
    };

    html! {
        <div class="card" style="height:100%;display:flex;flex-direction:column">
            <div class="card-image">
                <figure class="image is-3by2">
                    <img src={props.src.clone()}/>
                </figure>
            </div>
            <div class="card-content" style="height:100%">
                <div class="media">
                    <div class="media-content" style="overflow:hidden">
                        <p class="title is-4"> { props.name.clone() } </p>
                        <p class="subtitle is-6"> { props.creator.clone() } </p>
                    </div>
                </div>
                <div class="content">
                    { props.description.clone() }
                </div>
            </div>
            <div class="card-footer">
                { footer }
            </div>
        </div>
    }
}
