use cobul::props::ColumnSize;
use cobul::{Column, Columns, Container, Loading, Section};
use futures::prelude::*;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::{History, RouterScopeExt};

use crate::components::{MainNavbar, QuizCard};
use crate::graphql::{quizzes, Quiz};
use crate::shared::{Error, User, IMAGE_ENDPOINT, IMAGE_PLACEHOLDER};
use crate::{Route, UserAgent};

pub struct Overview {
    quizzes: Option<Vec<Quiz>>,
    agent: Box<dyn Bridge<UserAgent>>,
}

pub enum Msg {
    Quizzes(Result<Vec<Quiz>, Error>),
    User(Option<User>),
    Bonk(User),
}

impl Component for Overview {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let user = ctx.link().context::<User>(ctx.link().callback(Msg::Bonk)).map(|(user, _)| user);

        ctx.link().send_future(quizzes(user).map(Msg::Quizzes));
        Self { quizzes: None, agent: UserAgent::bridge(ctx.link().callback(Msg::User)) }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Quizzes(Ok(quizzes)) => {
                self.quizzes = Some(quizzes);
                true
            }
            Msg::Bonk(t) => {
                log::error!("it does le callback whee {:?}", t);
                false
            }
            Msg::Quizzes(Err(err)) => {
                log::error!("http error: {:?}", err);
                false
            }
            Msg::User(Some(_)) => {
                // let x = ctx.link().history().unwrap().route();
                ctx.link().history().unwrap().replace(Route::Overview);
                false
            }
            Msg::User(_) => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_quiz_card = |quiz: &Quiz| {
            let Quiz { title, description, image, id, creator, .. } = quiz.clone();

            let image = image
                .as_ref()
                .map(|path| format!("{}/{}", IMAGE_ENDPOINT, path))
                .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

            html! {
                <Column size={ColumnSize::Is3}>
                    <QuizCard {id} {title} {description} {image} {creator}/>
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
