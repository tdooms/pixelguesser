use yew::prelude::*;

use cobul::*;
use shared::{Player, Session, SessionDiff, Stage, Status};

use crate::graphql::{Quiz, Round};

use super::{Initialize, Master, Navigate, Rating};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<SessionDiff>,

    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
}

pub fn add_player(name: String, mut players: Vec<Player>) -> SessionDiff {
    match players.iter().find(|p| p.name == name) {
        None => {
            players.push(Player { name, score: 0 });
            SessionDiff { stage: None, players: Some(players) }
        }
        Some(_) => SessionDiff::default(),
    }
}

pub fn add_guess(name: String, mut players: Vec<Player>) -> SessionDiff {
    match players.iter_mut().find(|p| p.name == name) {
        Some(mut player) => {
            player.score += 1;
            SessionDiff { stage: None, players: Some(players) }
        }
        None => SessionDiff::default(),
    }
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let clone1 = props.session.players.clone();
    let clone2 = clone1.clone();

    let onadd = props.onchange.reform(move |name| add_player(name, clone1.clone()));
    let onguess = props.onchange.reform(move |name| add_guess(name, clone2.clone()));
    let onchange = props.onchange.reform(|stage| SessionDiff { players: None, stage: Some(stage) });

    let body = match props.session.stage {
        Stage::Initial => {
            html! { <Initialize onchange={onadd}/> }
        }
        Stage::Round { round, status: Status::Playing { .. } } => {
            html! { <Master players={props.session.players.clone()} onguess={onguess}/> }
        }
        Stage::Round { .. } => {
            html! { <TitleHero title="revealing" subtitle=""/> }
        } // TODO: don't show when revealed
        Stage::Ranking { .. } => {
            html! { <TitleHero title="showing scores" subtitle=""/> }
        }
        Stage::Finished => {
            html! { <Rating quiz={props.quiz.clone()} />}
        }
    };

    html! {
        <Section>
            <Container>
                { body }
                <Navigate stage={props.session.stage.clone()} rounds={props.rounds.len()} onchange={onchange}/>
            </Container>
        </Section>
    }
}
