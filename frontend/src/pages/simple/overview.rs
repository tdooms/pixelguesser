use yew::prelude::*;

use cobul::props::ColumnSize;
use cobul::*;

use crate::components::{MainNavbar, QuizCard};
use crate::constants::{IMAGE_ENDPOINT, IMAGE_PLACEHOLDER};
use crate::error::Error;
use crate::graphql::{quizzes, Quiz};
use crate::route::Route;

pub struct Overview {
    quizzes: Option<Vec<Quiz>>,
}

impl Component for Overview {
    type Message = Result<Vec<Quiz>, Error>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(quizzes());
        Self { quizzes: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(quizzes) => {
                self.quizzes = Some(quizzes);
                true
            }
            Err(err) => {
                log::error!("http error: {:?}", err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_quiz_card = |quiz: Quiz| {
            let src = quiz
                .image_url
                .as_ref()
                .map(|path| format!("{}/{}", IMAGE_ENDPOINT, path))
                .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());
            let route = Route::Host { quiz_id: quiz.quiz_id };

            html! {
                <Column size={ColumnSize::Is3}>
                    <QuizCard name={quiz.name} creator={quiz.creator} description={quiz.description} src={src} route={route}/>
                </Column>
            }
        };
        let view_quiz_cards = |chunk: &[Quiz]| {
            html! {
                <Columns>
                    { for chunk.iter().cloned().map(view_quiz_card) }
                </Columns>
            }
        };

        let view_quizzes = |quizzes: &Vec<Quiz>| {
            html! {
                <>
                <MainNavbar/>
                <Section>
                    <Container>
                        { for quizzes.chunks(4).map(view_quiz_cards) }
                    </Container>
                </Section>
                </>
            }
        };

        match &self.quizzes {
            Some(vec) => view_quizzes(vec),
            None => html! { <Loading /> },
        }
    }
}
