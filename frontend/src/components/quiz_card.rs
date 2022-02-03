use crate::Auth;
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

    pub public: bool,
}

#[function_component(QuizCard)]
pub fn quiz_card(props: &Props) -> Html {
    let Props { id, image, title, description, creator, .. } = props;
    let history = use_history().unwrap();

    let onclick = match *id {
        Some(quiz_id) => Callback::from(move |_| history.push(Route::Host { quiz_id })),
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

    let right = match (*id, use_context::<Auth>().unwrap()) {
        (Some(quiz_id), Auth::User(user)) if user.sub == creator.id => {
            let history = use_history().unwrap();
            let onclick = Callback::from(move |_| history.push(Route::Update { quiz_id }));

            html! { <Button {onclick} color={Color::White}> <Icon icon={Icons::EditRegular}/> </Button> }
        }
        _ => html! {},
    };

    let image = html_nested! { <Image size={ImageSize::Is3by2} src={image.clone()} /> };

    html! {
        <Card image={image} footer={footer} fullheight=true>
            <Media {right}>
                <p class="title is-4"> { title.clone() } </p>
                <p class="subtitle is-6"> { creator.name.clone() } </p>
            </Media>

            <Content>
                { description.clone() }
            </Content>
        </Card>
    }
}
