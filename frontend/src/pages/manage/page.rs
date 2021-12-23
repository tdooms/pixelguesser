use cobul::props::Color;
use cobul::*;
use yew::prelude::*;

use sessions::{Action, Session, Stage};

use crate::graphql::{Quiz, Round};

use super::{Navigate, PlayerForm, PlayerList, Rating, RoundInfo};

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

    let guessed = |round| callback.reform(|player| Action::Guessed(player, rounds[round].score));

    let body = match session.stage {
        Stage::Lobby => {
            html! {<PlayerForm callback={callback.reform(Action::Player)}/>}
        }
        Stage::Playing { round, paused } => {
            let onguess = callback.reform(|name| Action::Guessed(name, rounds[round].score));
            html! {
                <>
                <RoundInfo index={round} rounds={rounds.len()} round={rounds[round]}/>
                <PlayerList players={session.players.clone()} onguess={onguess}/>
                </>
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
    };

    html! {
        <Section>
            <Container>
                { body }
                <Navigate stage={props.session.stage.clone()} rounds={props.rounds.len()} callback={callback}/>
            </Container>
        </Section>
    }
}
