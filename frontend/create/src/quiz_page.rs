use cobul::*;
use yew::*;

use api::{Creator, DraftQuiz};
use components::QuizCard;
use shared::Auth;
use ywt::callback;

use crate::quiz_form::QuizForm;
use crate::state::QuizAction;
use crate::Stage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub onchange: Callback<QuizAction>,
    pub quiz: DraftQuiz,
    pub has_delete: bool,
}

#[function_component(QuizPage)]
pub fn quiz_page(props: &Props) -> Html {
    let Props { onstage, onchange, quiz, has_delete } = props.clone();
    let DraftQuiz { title, description, image, public, .. } = quiz.clone();

    let creator: Creator = match use_context::<Auth>().unwrap().user() {
        Ok(user) => user.into(),
        Err(_) => return html! { "not allowed" },
    };

    let onsubmit = callback!(onchange, onstage; move |_| {
        onchange.emit(QuizAction::Submit);
        onstage.emit(Stage::Rounds);
    });
    let ondelete = callback!(onchange, onstage; move |_| {
        onchange.emit(QuizAction::Delete);
        onstage.emit(Stage::Back);
    });
    let onback = callback!(onstage; move |_| onstage.emit(Stage::Back));
    let onchange = callback!(onchange; move |quiz| onchange.emit(QuizAction::Edit(quiz)));

    let delete = || html! {<Button color={Color::Danger} onclick={ondelete}> {"Delete"} </Button>};
    let left = html! {<Title> {"Overview"} </Title>};
    let right = has_delete.then(|| delete()).unwrap_or_default();

    html! {
        <Section>
        <Container>
            <Columns>
                <Column>
                    <Level {left} {right} />
                    <QuizForm {quiz} {onsubmit} {onback} {onchange}/>
                </Column>
                <Column size={ColumnSize::Is1} />
                <Column size={ColumnSize::Is4}>
                    <QuizCard {title} {description} {image} {creator} {public}/>
                </Column>
            </Columns>
        </Container>
        </Section>
    }
}
