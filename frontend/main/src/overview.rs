use cobul::props::ColumnSize;
use cobul::{Column, Columns, Container, Section};
use yew::prelude::*;
use yew::HtmlResult;

use api::{Quiz, IMAGE_ENDPOINT, IMAGE_PLACEHOLDER};
use components::MainNavbar;
use components::QuizCard;
use shared::use_quizzes;
use shared::Auth;

#[function_component(Overview)]
pub fn overview() -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user().ok();
    let quizzes = use_quizzes(user)?;

    let view_quiz_card = |quiz: &Quiz| {
        let Quiz { title, description, image, id, creator, public, .. } = quiz.clone();

        let image = image
            .as_ref()
            .map(|x| format!("{IMAGE_ENDPOINT}/{x}"))
            .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

        html! {
            <Column size={ColumnSize::Is3}>
                <QuizCard {id} {title} {description} {image} {creator} {public}/>
            </Column>
        }
    };

    Ok(html! {
        <>
        <MainNavbar/>
        <Section>
            <Container>
            <Columns multiline=true>
                { for quizzes.iter().map(view_quiz_card) }
            </Columns>
            </Container>
        </Section>
        </>
    })
}
