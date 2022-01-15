use cobul::props::ColumnSize;
use cobul::*;
use yew::prelude::*;

use crate::components::{MainNavbar, QuizCard};
use crate::constants::{IMAGE_ENDPOINT, IMAGE_PLACEHOLDER};
use crate::error::Error;
use crate::graphql::{quizzes, Quiz};

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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_quiz_card = |quiz: Quiz| {
            let Quiz { name, creator, description, image, quiz_id, .. } = quiz;

            let src = image
                .as_ref()
                .map(|path| format!("{}/{}", IMAGE_ENDPOINT, path))
                .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

            html! {
                <Column size={ColumnSize::Is3}>
                    <QuizCard id={quiz_id} name={name} creator={creator} description={description} image={src}/>
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
