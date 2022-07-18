use api::{Creator, DraftQuiz, Image, Quiz, Resolution, IMAGE_PLACEHOLDER};
use cobul::*;
use shared::Auth;
use std::rc::Rc;
use yew::*;
use yew_router::prelude::*;

use shared::Route;

#[derive(PartialEq, Debug)]
pub enum View {
    Normal { quiz: Quiz, onclick: Callback<()>, onedit: Callback<()> },
    Preview { draft: Rc<DraftQuiz>, creator: String },
    Uploaded { draft: Rc<DraftQuiz>, creator: String },
    Available { draft: Rc<DraftQuiz>, creator: String, onclick: Callback<()> },
    Empty,
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub view: View,
}

#[function_component(QuizCard)]
pub fn quiz_card(props: &Props) -> Html {
    let button = |color, pointer: bool, icon, text, onclick: Option<&Callback<()>>| {
        let radius = "border-top-left-radius: 0 !important;border-top-right-radius: 0 !important";
        let style = format!("{radius};{}", (!pointer).then(|| "cursor: unset").unwrap_or_default());

        let onclick = Callback::noop();
        html! {
            <Button {color} fullwidth=true {style} {onclick}>
                <Icon {icon}/> <span>{text}</span>
            </Button>
        }
    };

    let footer = match &props.view {
        View::Normal { onclick, .. } => {
            button(Color::Success, true, Icons::Play, "Play", Some(onclick))
        }
        View::Preview { .. } | View::Empty => {
            button(Color::Success, false, Icons::Play, "Play", None)
        }
        View::Uploaded { .. } => button(Color::Success, true, Icons::Check, "Uploaded", None),
        View::Available { onclick, .. } => {
            button(Color::Info, true, Icons::Upload, "Upload", Some(onclick))
        }
    };

    let right = match (&props.view, use_context::<Auth>().unwrap().user()) {
        (View::Normal { quiz, onedit, .. }, Ok(user)) if user.sub == quiz.creator.id => {
            html! { <Button onclick={onedit} color={Color::White}> <Icon icon={Icons::EditRegular}/> </Button> }
        }
        _ => html! {},
    };

    let empty = std::iter::repeat(' ').take(20).collect::<String>();
    let (src, title, creator, description) = match &props.view {
        View::Normal { quiz, .. } => {
            (quiz.image.src(Resolution::Card), &quiz.title, &quiz.creator.name, &quiz.description)
        }
        View::Preview { draft, creator }
        | View::Uploaded { draft, creator }
        | View::Available { draft, creator, .. } => {
            (draft.image.src(Resolution::Card), &draft.title, creator, &draft.description)
        }
        View::Empty => (Rc::from(IMAGE_PLACEHOLDER.to_owned()), &empty, &empty, &empty),
    };
    let image = html! { <cobul::Image size={ImageSize::Is3by2} src={(*src).clone()} /> };

    html! {
        <Card {image} {footer} fullheight=true>
            <Media {right}>
                <p class="title is-4"> { title } </p>
                <p class="subtitle is-6"> { creator } </p>
            </Media>

            <Content>
                { description }
            </Content>
        </Card>
    }
}
