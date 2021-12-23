use cobul::props::Color;
use cobul::*;
use yew::prelude::*;

use crate::graphql::{Quiz, Round};
use crate::route::Route;
use sessions::{Action, Session, Stage};
use yew_router::prelude::*;

use super::{Navigate, PlayerForm, PlayerList, PlayerMsg, PlayerName, Rating, RoundInfo};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,

    pub callback: Callback<Action>,
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let Props { session, quiz, rounds, callback } = props;

    let state = use_state(|| PlayerName::default());

    let form_callback = {
        let state = state.clone();
        let cloned = callback.clone();

        Callback::from(move |msg| match msg {
            PlayerMsg::Submit(player) => cloned.emit(Action::Player(player.name)),
            PlayerMsg::Change(player) => state.set(player),
        })
    };

    let body = match session.stage {
        Stage::Lobby => {
            html! {<PlayerForm inner={(*state).clone()} callback={form_callback}/>}
        }
        Stage::Playing { round, paused } => {
            let points = rounds[round].info.points as u64;
            let onguess = callback.reform(move |name| Action::Guessed(name, points));
            html! {
                <div>
                    <RoundInfo index={round} rounds={rounds.len()} round={rounds[round].clone()}/>
                    <PlayerList players={session.players.clone()} onguess={onguess}/>
                </div>
            }
        }
        Stage::Revealed { round } => {
            html! { <Hero color={Color::Primary}><Title> {format!("End of round {}", round)} </Title> </Hero> }
        }
        Stage::Ranking { .. } => {
            html! { <Hero color={Color::Primary}><Title> {"Scores"} </Title> </Hero> }
        }
        Stage::Finished => {
            html! { <Rating quiz={props.quiz.clone()} />}
        }
        Stage::Left => {
            use_history().unwrap().push(Route::Overview);
            html! {}
        }
    };

    // TODO: this is probably a yew bug but the body appears under the buttons...
    html! {
        <Section>
            <Container>
                { body }
                <Navigate session={props.session.clone()} rounds={props.rounds.len()} callback={callback}/>
            </Container>
        </Section>
    }
}
