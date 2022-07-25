use cobul::*;
use yew::*;

use super::Ranking;
use api::{Quiz, Session};
use shared::Route;
use std::rc::Rc;
use yew_router::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub session: Rc<Session>,
    pub quiz: Rc<Quiz>,
}

#[function_component(Finish)]
pub fn finish(props: &Props) -> Html {
    let Props { session, quiz } = &props;
    let navigator = use_navigator().unwrap().clone();
    let onleave = { Callback::from(move |_| navigator.push(Route::Overview)) };

    html! {
        <>
            <Hero color={Color::Primary}>
                <Title> {quiz.title.clone()} </Title>
                <Subtitle> {"finished"} </Subtitle>
            </Hero>

            <Hero color={Color::Primary} size={HeroSize::Medium}>
                <Ranking players={session.players.clone()} />
            </Hero>

        <Buttons alignment={Alignment::Centered} class="mt-5">
            <Button color={Color::Primary} light=true onclick={onleave} size={Size::Large}>
                <Icon icon={Solid::RightFromBracket}/> <span> {"leave"} </span>
            </Button>
        </Buttons>
        </>
    }
}
