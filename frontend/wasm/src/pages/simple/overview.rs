use yew::prelude::*;
use yew::utils::NeqAssign;

use graphql::{Quiz, quizzes};
use reqwasm::Error;
use cbs::Loading;

use crate::components::{navbar, QuizCard};
use crate::route::Route;
use crate::constants::IMAGE_ENDPOINT;

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
            Err(_) => {} // TODO: error
        }

    }

    fn change(&mut self, props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let view_quiz_card = |quiz: Quiz| {
            let url = format!("{}/{}", IMAGE_ENDPOINT, quiz.image_url.unwrap_or("".to_owned()));
            let route = Route::Host { quiz_id: quiz.quiz_id };
            html! {
                <div class="column is-3">
                    <QuizCard name={quiz.name} creator={quiz.creator} description={quiz.description} image_url={url} route={route}/>
                </div>
            }
        };
        let view_quiz_cards = |chunk: &[Quiz]| {
            html! {
                <div class="columns">
                    { for chunk.iter().cloned().map(view_quiz_card) }
                </div>
            }
        };

        let view_quizzes = |quizzes: &Vec<Quiz>| {
            html! {
                <>
                { navbar() }
                <section class="section">
                    <div class="container">
                        { for quizzes.chunks(4).map(view_quiz_cards) }
                    </div>
                </section>
                </>
            }
        };

        match self.quizzes.as_ref() {
            Some(quizzes) => view_quizzes(quizzes),
            None => html! { <Loading/> },
        }
    }
}
