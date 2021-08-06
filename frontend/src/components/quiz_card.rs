use yew::prelude::*;
use yew_router::components::Link;
use yewtil::NeqAssign;

use crate::route::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub image_url: Option<String>,

    pub name: String,

    pub creator: String,

    pub description: String,

    #[prop_or_default]
    pub route: Option<Route>,
}

pub struct QuizCard {
    props: Props,
}

impl Component for QuizCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let src = match &self.props.image_url {
            Some(url) => url.clone(),
            None => "https://styles.io/images/placeholders/480x320.png".to_owned(),
        };

        let footer = match &self.props.route {
            Some(route) => html! {
                <Link<Route> classes={classes!("button", "is-success", "is-fullwidth", "has-square-top")} route={route.clone()}>
                    <span class="icon"><i class="fas fa-play"></i></span> <strong>{"Play"}</strong>
                </Link<Route>>
            },
            None => html! {
                <button class=classes!("button", "is-success", "is-fullwidth", "has-square-top")>
                    <span class="icon"><i class="fas fa-play"></i></span> <strong>{"Play"}</strong>
                </button>
            },
        };

        html! {
            <div class="card" style="height:100%;display:flex;flex-direction:column">
                <div class="card-image">
                    <figure class="image is-3by2">
                        <img src={src}/>
                    </figure>
                </div>
                <div class="card-content" style="height:100%">
                    <div class="media">
                        <div class="media-content" style="overflow:hidden">
                            <p class="title is-4"> { self.props.name.clone() } </p>
                            <p class="subtitle is-6"> { self.props.creator.clone() } </p>
                        </div>
                    </div>
                    <div class="content">
                        { self.props.description.clone() }
                    </div>
                </div>
                <div class="card-footer">
                    { footer }
                </div>
            </div>
        }
    }
}
