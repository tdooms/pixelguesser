use cobul::props::Color;
use cobul::*;
use yew::prelude::*;

use crate::graphql::{Quiz, Round};
use crate::shared::Route;
use sessions::{Action, Session, Stage};
use yew_router::prelude::*;

use super::{Navigate, PlayerForm, PlayerList, PlayerName, Rating, RoundInfo};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,

    pub callback: Callback<Action>,
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let Props { session, quiz: _, rounds, callback } = props;

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

    let body = match props.session.stage {
        Stage::Lobby => {
            let onremove = props.callback.reform(Action::RemovePlayer);
            let title = "Select a player to remove them.";

            html! {
                <>
                    <PlayerForm inner={(*state).clone()} onchange={onchange} onsubmit={onsubmit}/>
                    <PlayerList title={title} players={session.players.clone()} onclick={onremove}/>
                </>
            }
        }
        Stage::Playing { round, paused: _, revealed: false } => {
            let points = rounds[round].points as i64;
            let onguess = props.callback.reform(move |name| Action::CorrectGuess(name, points));
            let title = "Select the player who guessed correctly.";

            html! {
                <>
                    <RoundInfo index={round} rounds={rounds.len()} round={rounds[round].clone()}/>
                    <PlayerList title={title} players={session.players.clone()} onclick={onguess}/>
                </>
            }
        }
        Stage::Playing { round, paused: _, revealed: true } => html! {
            <Hero color={Color::Primary}><Title> {format!("End of round {}", round + 1)} </Title> </Hero>
        },
        Stage::Finished => html! {
            <Rating quiz={props.quiz.clone()} />
        },
        Stage::Left => {
            use_history().unwrap().push(Route::Overview);
            html! {}
        }
    };

    html! {
        <Section>
            <Container>
                { body }
                <Navigate session={props.session.clone()} rounds={props.rounds.len()} callback={callback}/>
            </Container>
        </Section>
    }
}
