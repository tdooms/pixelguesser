use yew::prelude::*;
use yew_router::components::Link;

use cobul::props::{Color, ImageSize};
use cobul::*;

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
            <Link<Route> classes={classes!("button", "is-success", "is-fullwidth", "has-square-top")} to={route.clone()}>
                <Icon icon={"fas fa-play"}/> <span>{"Play"}</span>
            </Link<Route>>
        },
        None => html! {
            <Button color={Color::Success} fullwidth=true extra={"has-square-top has-no-pointer"}>
                <Icon icon={"fas fa-play"}/> <span>{"Play"}</span>
            </Button>
        },
    };

    let image = html_nested! { <Image size={ImageSize::Is3by2} src={props.src.clone()} /> };

    html! {
        <Card image={image} footer={footer} fullheight=true>
            <Media>
                <p class="title is-4"> { props.name.clone() } </p>
                <p class="subtitle is-6"> { props.creator.clone() } </p>
            </Media>

            <Content>
                { props.description.clone() }
            </Content>
        </Card>
    }
}
