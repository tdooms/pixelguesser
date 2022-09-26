use api::{Quiz, Resolution, IMAGE_PLACEHOLDER};
use cobul::fa::Solid;
use cobul::simple::Button;
use cobul::*;
use shared::use_auth;
use std::rc::Rc;
use yew::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    quiz: Option<Rc<Quiz>>,

    #[prop_or_default]
    creator: Option<String>,

    #[prop_or_default]
    play: Callback<()>,

    #[prop_or_default]
    edit: Callback<()>,
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ButtonProps {
    color: Color,
    text: String,
    icon: String,

    pointer: bool,
    click: Callback<()>,
}

#[function_component(QuizButton)]
pub fn quiz_button(props: &ButtonProps) -> Html {
    let ButtonProps { color, pointer, text, icon, click } = props.clone();

    let radius = "border-top-left-radius: 0 !important;border-top-right-radius: 0 !important";
    let style = format!("{radius};{}", (!pointer).then(|| "cursor: unset").unwrap_or_default());

    html! { <Button {color} fullwidth=true {style} {click} {icon} {text} /> }
}

#[function_component(QuizCard)]
pub fn quiz_card(props: &Props) -> Html {
    let Props { quiz, play, edit, .. } = props;

    let creator = quiz.as_ref().map(|x| x.creator.clone()).flatten();

    let creator_id = creator.as_ref().map(|x| x.id.clone()).flatten();
    let user_id = use_auth().user().map(|x| x.id.clone()).flatten();

    let footer = html! { <QuizButton color={Color::Success} pointer=true icon={Solid::Play} text={"Play"} click={play} /> };

    let right = match creator_id == user_id {
        true => html! { <Button click={edit} color={Color::White} icon={Solid::PenToSquare} /> },
        false => html! {},
    };

    let filler = |n| std::iter::repeat(" x").take(n).collect::<String>();

    let title = quiz.as_ref().map(|x| x.title.clone()).unwrap_or_else(|| filler(5));
    let description = quiz.as_ref().map(|x| x.description.clone()).unwrap_or_else(|| filler(15));
    let creator = creator.map(|x| x.nickname).unwrap_or_else(|| filler(5));
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
                <p class="subtitle is-6" {style}> { creator } </p>
            </Media>

            <Content>
                <p {style}> { description } </p>
            </Content>
        </Card>
    }
}
