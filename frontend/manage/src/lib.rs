use std::rc::Rc;

use api::{Action, Participant, Phase, Quiz, Session, Stage};
use cobul::*;
use shared::Route;
use yew::*;
use yew_router::prelude::*;

use crate::navigate::Navigate;
use crate::player_form::PlayerForm;
use crate::player_list::PlayerList;
use crate::rating::Rating;
use crate::round_info::RoundInfo;

mod navigate;
mod player_form;
mod player_list;
mod rating;
mod round_info;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Rc<Session>,
    pub quiz: Rc<Quiz>,

    pub callback: Callback<Action>,
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let Props { session, quiz, callback } = props;

    let onsubmit = callback.reform(Action::Add);
    let onremove = callback.reform(Action::Remove);

    let onguess = |round: usize, quiz: &Quiz| {
        let points = quiz.rounds[round].points as i64;
        callback.reform(move |name| Action::Score(name, points))
    };

    if !session.participants.contains_key(&Participant::Host) {
        log::error!("host has left")
    }

    let onleave = {
        let navigator = use_navigator().unwrap().clone();
        Callback::from(move |_| navigator.push(Route::Overview))
    };

    let rounds = quiz.rounds.len();
    let body = match session.phase {
        Phase::Playing { round, .. } if round >= rounds => {
            log::error!("empty quiz");
            html! {}
        }
        Phase::Lobby => html! {
            <>
            <PlayerForm {onsubmit}/>
            <Block/>
            <PlayerList title={"Select a player to remove them."} {session} onclick={onremove}/>
            <Navigate {session} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Playing { stage: Stage::Info, round } => html! {
            <RoundInfo index={round} rounds={quiz.rounds.len()} round={quiz.rounds[round].clone()}/>
        },
        Phase::Playing { stage: Stage::Running | Stage::Paused, round } => html! {
            <>
            <RoundInfo index={round} rounds={quiz.rounds.len()} round={quiz.rounds[round].clone()}/>
            <PlayerList title={"Select the player who guessed correctly."} {session} onclick={onguess(round, &quiz)}/>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Playing { stage: Stage::Revealing, .. } => html! {
            <Hero color={Color::Primary}> <Title> {"Revealing ..."} </Title> </Hero>
        },
        Phase::Playing { stage: Stage::Scores, .. } => html! {
            <>
            <Hero color={Color::Primary}> <Title> {"Showing scores"} </Title> </Hero>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Playing { round, stage: Stage::Revealed } => html! {
            <>
            <Hero color={Color::Primary}>
                <Title> {format!("End of round {}", round + 1)} </Title>
            </Hero>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Finished => html! {
            <>
            <Rating {quiz} />
            <Buttons alignment={Alignment::Centered}>
                <Button color={Color::Primary} light=true onclick={onleave} size={Size::Large}>
                    <Icon icon={fa::Solid::RightFromBracket}/> <span> {"leave"} </span>
                </Button>
            </Buttons>
            </>
        },
    };

    html! {
        <Section>
            <Container>
                { body }
            </Container>
        </Section>
    }
}
