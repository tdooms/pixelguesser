use yew::prelude::*;
use yew::utils::NeqAssign;

use api::{Fetch, Get, Quiz, Request, Response};
use cbs::Loading;

use crate::agents::WebSocketAgent;
use crate::components::{navbar, QuizCard};
use crate::route::Route;
use crate::utils::{exec_query, image_url, OVERVIEW_QUERY, Quiz};

pub struct Overview {
    quizzes: Option<Vec<Quiz>>,
}

impl Component for Overview {
    type Message = Vec<Quiz>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_future(exec_query(OVERVIEW_QUERY, |x| x));
        Self { quizzes: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.quizzes.neq_assign(Some(msg))
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let view_quiz_card = |quiz: Quiz| {
            let url = image_url(quiz.image_url);
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
