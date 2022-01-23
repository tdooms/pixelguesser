use cobul::props::ColumnSize;
use cobul::{Column, Columns, Container, Loading, Section};
use futures::prelude::*;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::{History, RouterScopeExt};

use crate::components::{MainNavbar, QuizCard};
use crate::graphql::{quizzes, Quiz};
use crate::shared::{Error, IMAGE_ENDPOINT, IMAGE_PLACEHOLDER};
use crate::{Route, User, UserAgent};

pub struct Overview {
    quizzes: Option<Vec<Quiz>>,
    agent: Box<dyn Bridge<UserAgent>>,
}

pub enum Msg {
    Quizzes(Result<Vec<Quiz>, Error>),
    User(User),
}

impl Component for Overview {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(quizzes().map(Msg::Quizzes));
        Self { quizzes: None, agent: UserAgent::bridge(ctx.link().callback(Msg::User)) }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Quizzes(Ok(quizzes)) => {
                self.quizzes = Some(quizzes);
                true
            }
            Msg::Quizzes(Err(err)) => {
                log::error!("http error: {:?}", err);
                false
            }
            Msg::User(User::User(_)) => {
                ctx.link().history().unwrap().replace(Route::Overview);
                false
            }
            Msg::User(_) => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_quiz_card = |quiz: &Quiz| {
            let Quiz { name, creator, description, image, quiz_id, .. } = quiz.clone();

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

        let view_quizzes = |quizzes: &Vec<Quiz>| {
            html! {
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
            }
        };

        match &self.quizzes {
            Some(vec) => view_quizzes(vec),
            None => html! { <Loading /> },
        }
    }
}
