use std::rc::Rc;

use cobul::*;
use yew::*;
use yew_router::prelude::*;

use api::{Quiz, Session};
use shared::Route;

use super::Ranking;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub session: Rc<Session>,
    pub quiz: Rc<Quiz>,
}

#[function_component(Finish)]
pub fn finish(props: &Props) -> Html {
    let Props { session, quiz } = &props;
    let navigator = use_navigator().unwrap().clone();
    let leave = { Callback::from(move |_| navigator.push(&Route::Overview)) };

    html! {
        <>
            <Hero color={Color::Info}>
                <Title> {quiz.title.clone()} </Title>
                <Subtitle> {"finished"} </Subtitle>
            </Hero>

            <Hero color={Color::Info} size={HeroSize::Medium}>
                <Ranking players={session.players.clone()} />
            </Hero>

        <Buttons alignment={Alignment::Centered} class="mt-5">
            <simple::Button color={Color::Info} light=true click={leave} size={Size::Large} icon={fa::Solid::RightFromBracket} text="leave" />
        </Buttons>
        </>
    }
}
