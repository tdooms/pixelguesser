use api::{DraftQuiz, Quiz, Resolution, IMAGE_PLACEHOLDER};
use cobul::*;
use shared::use_auth;
use std::rc::Rc;
use yew::*;

#[derive(PartialEq, Debug)]
pub enum View {
    Normal { quiz: Quiz, onclick: Callback<()>, onedit: Callback<()> },
    Preview { draft: Rc<DraftQuiz>, creator: String },
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
        let onclick = onclick.cloned().unwrap_or_default();

        html! {
            <Button {color} fullwidth=true {style} {onclick}>
                <Icon {icon}/> <span>{text}</span>
            </Button>
        }
    };

    let footer = match &props.view {
        View::Normal { onclick, .. } => {
            button(Color::Success, true, fa::Solid::Play, "Play", Some(onclick))
        }
        View::Preview { .. } | View::Empty => {
            button(Color::Success, false, fa::Solid::Play, "Play", None)
        }
    };

    let right = match (&props.view, use_auth().user()) {
        (View::Normal { quiz, onedit, .. }, Some(user)) if user.sub == quiz.creator.id => {
            html! { <Button onclick={onedit} color={Color::White}> <Icon icon={fa::Solid::PenToSquare}/> </Button> }
        }
        _ => html! {},
    };

    let filler = |n| std::iter::repeat(" x").take(n).collect::<String>();
    let (small, large) = (filler(5), filler(15));

    let (src, title, creator, description) = match &props.view {
        View::Normal { quiz, .. } => {
            (quiz.image.src(Resolution::Small), &quiz.title, &quiz.creator.name, &quiz.description)
        }
        View::Preview { draft, creator } => {
            (draft.image.src(Resolution::Small), &draft.title, creator, &draft.description)
        }
        View::Empty => (Rc::from(IMAGE_PLACEHOLDER.to_owned()), &small, &small, &large),
    };
    let style = (props.view == View::Empty).then(|| "visibility:hidden");
    let image = html! { <cobul::Image size={ImageSize::Is3by2} src={(*src).clone()} /> };

    html! {
        <Card {image} {footer} fullheight=true>
            <Media {right}>
                <p class="title is-4" {style}> { title } </p>
                <p class="subtitle is-6" {style}> { creator } </p>
            </Media>

            <Content>
                <p {style}> { description } </p>
            </Content>
        </Card>
    }
}
