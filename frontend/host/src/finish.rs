use yew::prelude::*;

use cobul::props::{Color, HeroSize};
use cobul::*;

use super::Ranking;
use api::{FullQuiz, Session};
use shared::Route;
use std::rc::Rc;
use yew_router::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub session: Rc<Session>,
    pub quiz: Rc<FullQuiz>,
}

#[function_component(Finish)]
pub fn finish(props: &Props) -> Html {
    let Props { session, quiz } = &props;

    let onleave = {
        let history = use_history().unwrap().clone();
        Callback::from(move |_| history.push(Route::Overview))
    };

    html! {
        <>
            <Hero color={Color::Primary}>
                <Title> {quiz.title.clone()} </Title>
                <Subtitle> {"finished"} </Subtitle>
            </Hero>

            <Hero color={Color::Primary} size={HeroSize::Medium}>
                <Ranking {session} />
            </Hero>

            <Button color={Color::Primary} light=true onclick={onleave}>
                <Icon icon={Icons::SignOutAlt}/> <span> {"leave"} </span>
            </Button>
        </>
    }
}
