use cobul::props::{Color, ImageSize};
use cobul::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::graphql::Creator;
use crate::shared::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<u64>,

    pub image: String,

    pub title: String,

    pub description: String,

    pub creator: Creator,
}

#[function_component(QuizCard)]
pub fn quiz_card(props: &Props) -> Html {
    let Props { id, image, title, description, creator } = props;
    let history = use_history().unwrap();

    let onclick = match *id {
        Some(quiz_id) => Callback::from(move |_| history.push(Route::Host { quiz_id })),
        None => Callback::noop(),
    };

    let extra = match id {
        Some(_) => "has-square-top",
        None => "has-square-top has-no-pointer",
    };

    let footer = html! {
        <Button color={Color::Success} fullwidth=true class={extra} onclick={onclick}>
            <Icon icon={Icons::Play}/> <span>{"Play"}</span>
        </Button>
    };

    let right = |quiz_id| {
        let history = use_history().unwrap();
        let callback = Callback::from(move |_| history.push(Route::Update { quiz_id }));
        html! {
            <Button color={Color::White} onclick={callback}>
                <Icon icon={Icons::EditRegular}/>
            </Button>
        }
    };

    let image = html_nested! { <Image size={ImageSize::Is3by2} src={image.clone()} /> };

    html! {
        <Card image={image} footer={footer} fullheight=true>
            <Media right={id.map(right).unwrap_or_default()}>
                <p class="title is-4"> { title.clone() } </p>
                <p class="subtitle is-6"> { creator.name.clone() } </p>
            </Media>

            <Content>
                { description.clone() }
            </Content>
        </Card>
    }
}
