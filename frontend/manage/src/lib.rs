use std::rc::Rc;

use api::{Action, FullQuiz, Session, Stage};
use cobul::props::Color;
use cobul::*;
use shared::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::navigate::Navigate;
use crate::player_form::{PlayerForm, PlayerName};
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
    pub full: Rc<FullQuiz>,

    pub callback: Callback<Action>,
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let Props { session, full, callback } = props;

    let state = use_state(|| PlayerName::default());

    let onchange = {
        let state = state.clone();
        Callback::from(move |player| state.set(player))
    };
    let onsubmit = {
        let state = state.clone();
        callback.reform(move |player: PlayerName| {
            state.set(PlayerName::default());
            Action::AddPlayer(player.name)
        })
    };
    let onleave = {
        let history = use_history().unwrap().clone();
        Callback::from(move |_| history.push(Route::Overview))
    };

    let body = match props.session.stage {
        Stage::Lobby => {
            let onremove = props.callback.reform(Action::RemovePlayer);
            let title = "Select a player to remove them.";

            html! {
                <>
                <PlayerForm inner={(*state).clone()} onchange={onchange} onsubmit={onsubmit}/>
                <Block/>
                <PlayerList title={title} {session} onclick={onremove}/>
                <Navigate {session} rounds={full.rounds.len()} {callback}/>
                </>
            }
        }
        Stage::Playing { round, paused: _, revealing: false } => {
            let points = full.rounds[round].points as i64;
            let onguess = callback.reform(move |name| Action::CorrectGuess(name, points));
            let title = "Select the player who guessed correctly.";

            html! {
                <>
                <RoundInfo index={round} rounds={full.rounds.len()} round={full.rounds[round].clone()}/>
                <PlayerList title={title} {session} onclick={onguess}/>
                <Navigate session={session.clone()} rounds={full.rounds.len()} {callback}/>
                </>
            }
        }
        Stage::Playing { round, paused: _, revealing: true } => html! {
            <>
            <Hero color={Color::Primary}>
                <Title> {format!("End of round {}", round + 1)} </Title>
            </Hero>
            <Navigate session={session.clone()} rounds={full.rounds.len()} {callback}/>
            </>
        },
        Stage::Finished => html! {
            <>
            <Rating {full} />
            <Button color={Color::Primary} light=true onclick={onleave}>
                <Icon icon={Icons::SignOutAlt}/> <span> {"leave"} </span>
            </Button>
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
