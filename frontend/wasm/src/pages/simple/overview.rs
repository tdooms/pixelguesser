use yew::prelude::*;

use crate::graphql::{quizzes, Quiz};
use cbs::MaybeLoading;
use pbs::prelude::*;
use pbs::properties::ColumnSize;
use reqwasm::Error;

use crate::components::{Navbar, QuizCard};
use crate::constants::IMAGE_ENDPOINT;
use crate::route::Route;
use yew::utils::NeqAssign;

pub struct Overview {
    quizzes: Option<Vec<Quiz>>,
}

impl Component for Overview {
    type Message = Result<Vec<Quiz>, Error>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_future(quizzes());
        Self { quizzes: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Ok(quizzes) => self.quizzes.neq_assign(Some(quizzes)),
            Err(err) => {
                log::error!("http error: {:?}", err);
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let view_quiz_card = |quiz: Quiz| {
            let url = format!("{}/{}", IMAGE_ENDPOINT, quiz.image_url.unwrap_or("".to_owned()));
            let route = Route::Host { quiz_id: quiz.quiz_id };

            html! {
                <Column size={ColumnSize::Is3}>
                    <QuizCard name={quiz.name} creator={quiz.creator} description={quiz.description} image_url={url} route={route}/>
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
                <Navbar/>
                <Section>
                    <Container>
                        { for quizzes.chunks(4).map(view_quiz_cards) }
                    </Container>
                </Section>
                </>
            }
        };

        html! { <MaybeLoading html={self.quizzes.as_ref().map(view_quizzes)}/> }
    }
}
