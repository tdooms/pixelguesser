use crate::utils::code_to_string;
use api::{Player, Quiz, Round};
use pbs::{Color, ColumnSize, HeroSize};
use std::collections::HashMap;
use yew::prelude::*;

pub fn lobby(
    data: Option<&(u64, Quiz, Vec<Round>)>,
    has_manager: bool,
    players: &HashMap<u64, Player>,
) -> Html {
    let players = players.iter().map(|(_, player)| {
        html! {
            <pbs::Column size=ColumnSize::IsNarrow>
                <pbs::Box> {&player.name} </pbs::Box>
            </pbs::Column>
        }
    });

    let title = data
        .as_ref()
        .map(|(_, quiz, _)| quiz.name.clone())
        .unwrap_or_default();

    let subtitle = if has_manager {
        "quiz master present"
    } else {
        "no quiz master"
    };

    let code = data
        .map(|(id, _, _)| code_to_string(*id))
        .flatten()
        .unwrap_or_default();

    let body = html! {
        <pbs::Container extra="has-text-centered">
            <pbs::Title> {code} </pbs::Title>
        </pbs::Container>
    };

    html! {
        <>
            <pbs::SimpleHero title=title subtitle=subtitle />
            <pbs::Hero color=Color::Primary size=HeroSize::Medium body=body />

            <pbs::Columns multiline=true centered=true extra="mt-5">
                { for players }
            </pbs::Columns>
        </>
    }
}
