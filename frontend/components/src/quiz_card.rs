use std::rc::Rc;

use cobul::fa::Solid;
use cobul::*;
use yew::*;

use api::{Quiz, Resolution, IMAGE_PLACEHOLDER};
use shared::use_auth;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub quiz: Option<Rc<Quiz>>,

    #[prop_or_default]
    pub creator: Option<String>,

    #[prop_or_default]
    pub play: Callback<()>,

    #[prop_or_default]
    pub edit: Callback<()>,
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ButtonProps {
    pub color: Color,
    pub text: AttrValue,
    pub icon: AttrValue,

    pub pointer: bool,
    pub click: Callback<()>,
}

#[function_component(QuizButton)]
pub fn quiz_button(props: &ButtonProps) -> Html {
    let ButtonProps { color, pointer, text, icon, click } = props.clone();

    let radius = "border-top-left-radius: 0 !important;border-top-right-radius: 0 !important";
    let style = format!("{radius};{}", (!pointer).then(|| "cursor: unset").unwrap_or_default());

    html! { <simple::Button {color} fullwidth=true {style} {click} {icon} {text} /> }
}

#[function_component(QuizCard)]
pub fn quiz_card(props: &Props) -> Html {
    let Props { quiz, play, edit, .. } = props;

    let creator = quiz.as_ref().map(|x| x.creator.clone()).flatten();
    let user = use_auth().user();

    let creator_id = creator.as_ref().map(|x| x.id.clone()).flatten();
    let user_id = user.as_ref().map(|x| x.id.clone()).flatten();

    let footer = html! { <QuizButton color={Color::Success} pointer=true icon={Solid::Play} text={"Play"} click={play} /> };

    let right = (creator_id.is_some() && creator_id == user_id)
        .then(|| html! { <simple::Button click={edit} color={Color::White} icon={Solid::PenToSquare} /> })
        .unwrap_or_default();

    let filler = |n| std::iter::repeat(" x").take(n).collect::<String>();

    let title = quiz.as_ref().map(|x| x.title.clone()).unwrap_or_else(|| filler(5));
    let description = quiz.as_ref().map(|x| x.description.clone()).unwrap_or_else(|| filler(15));

    let nickname = match (user, creator) {
        (_, Some(creator)) => creator.nickname.clone(),
        (Some(user), None) => user.nickname.clone(),
        _ => filler(5),
    };

    let src = quiz
        .as_ref()
        .map(|x| x.image.src(Resolution::Small))
        .unwrap_or_else(|| Rc::from(IMAGE_PLACEHOLDER.to_owned()));

    let style = props.quiz.is_none().then(|| "visibility:hidden");
    let image = html! { <cobul::Image size={ImageSize::Is3by2} src={(*src).clone()} /> };

    html! {
        <Card {image} {footer} fullheight=true>
            <Media {right}>
                <p class="title is-4" {style}> { title } </p>
                <p class="subtitle is-6" {style}> { nickname } </p>
            </Media>

            <Content>
                <p {style}> { description } </p>
            </Content>
        </Card>
    }
}
