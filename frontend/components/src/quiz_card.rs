use api::{Creator, IMAGE_PLACEHOLDER};
use cobul::props::{Color, ImageSize};
use cobul::*;
use shared::Auth;
use yew::prelude::*;
use yew_router::prelude::*;

use shared::Route;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<u32>,
    #[prop_or_default]
    pub image: Option<String>,

    pub title: String,
    pub description: String,
    pub creator: Creator,
    pub public: bool,
}

#[function_component(QuizCard)]
pub fn quiz_card(props: &Props) -> Html {
    let Props { id, image, title, description, creator, .. } = props.clone();
    let navigator = use_navigator().unwrap();

    let onclick = match id {
        Some(quiz_id) => {
            let cloned = navigator.clone();
            Callback::from(move |_| cloned.push(Route::Host { quiz_id }))
        }
        None => Callback::noop(),
    };

    let class = match id {
        Some(_) => "has-square-top",
        None => "has-square-top has-no-pointer",
    };

    let footer = html! {
        <Button color={Color::Success} fullwidth=true {class} {onclick}>
            <Icon icon={Icons::Play}/> <span>{"Play"}</span>
        </Button>
    };

    let right = match (id, use_context::<Auth>().unwrap().user()) {
        (Some(quiz_id), Ok(user)) if user.sub == creator.id => {
            let onclick = Callback::from(move |_| navigator.push(Route::Update { quiz_id }));
            html! { <Button {onclick} color={Color::White}> <Icon icon={Icons::EditRegular}/> </Button> }
        }
        _ => html! {},
    };

    let src = image.unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());
    let image = html! { <Image size={ImageSize::Is3by2} {src} /> };

    html! {
        <Card {image} {footer} fullheight=true>
            <Media {right}>
                <p class="title is-4"> { title } </p>
                <p class="subtitle is-6"> { creator.name } </p>
            </Media>

            <Content>
                { description }
            </Content>
        </Card>
    }
}

#[function_component(EmptyCard)]
pub fn empty_card() -> Html {
    let footer = html! {
        <Button color={Color::Success} fullwidth=true class={"has-square-top"}>
            <Icon icon={Icons::Play}/> <span>{"Play"}</span>
        </Button>
    };

    let image = html! { <Image size={ImageSize::Is3by2} src={IMAGE_PLACEHOLDER.to_owned()} /> };
    let right = html! { <Icon icon={Icons::EditRegular}/> };

    let empty = std::iter::repeat(' ').take(20).collect::<String>();

    html! {
        <Card {image} {footer} fullheight=true>
            <Media {right}> <p class="title is-4"></p> <p class="subtitle is-6"> </p> </Media>
            <Content> <pre> {empty} </pre> </Content>
        </Card>
    }
}
