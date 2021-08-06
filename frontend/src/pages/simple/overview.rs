use yew::prelude::*;

use api::{Fetch, Get, Quiz, Request, Response};
use cbs::Loading;

use crate::agents::WebSocketAgent;
use crate::components::{navbar, QuizCard};
use crate::constants::IMAGE_ENDPOINT;
use crate::route::Route;

pub struct Overview {
    quizzes: Option<Vec<Quiz>>,
    _ws_agent: Box<dyn Bridge<WebSocketAgent>>,
}

impl Component for Overview {
    type Message = Response;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_agent = WebSocketAgent::bridge(link.callback(|x| x));
        ws_agent.send(Request::Get(Get::Quizzes));

        Self { quizzes: None, _ws_agent: ws_agent }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Response::Fetch(Fetch::Quizzes(quizzes)) => {
                self.quizzes = Some(quizzes);
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let view_quiz_card = |quiz: Quiz| {
            let url = format!("http://{}/{}", IMAGE_ENDPOINT, quiz.image_url);
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
