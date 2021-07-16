use crate::utils::code_to_string;
use api::{Player, Quiz, Round};
use bulma::Box;
use std::collections::HashMap;
use yew::prelude::*;

pub fn lobby(
    data: Option<&(u64, Quiz, Vec<Round>)>,
    has_manager: bool,
    players: &HashMap<u64, Player>,
) -> Html {
    let view_player = |(_, player): (&u64, &Player)| {
        html! {
            <div class="column is-narrow">
                <Box>
                    {&player.name}
                </Box>
            </div>
        }
    };

    html! {
        <>
            <section class="hero">
                <div class="hero-body">
                    <p class="title"> {data.as_ref().map(|(_, quiz, _)| quiz.name.clone()).unwrap_or_default()} </p>
                    <p class="subtitle"> {if has_manager {"quiz master present"} else {"no quiz master"}} </p>
                </div>
            </section>

            <section class="hero is-primary is-medium">
                <div class="hero-body">
                    <div class="container has-text-centered">
                        <p class="title"> { data.map(|(id, _, _)| code_to_string(*id)).flatten().unwrap_or_default() } </p>
                    </div>
                </div>
            </section>

            <div class="columns is-multiline is-centered mt-5">
                { for players.iter().map(view_player) }
            </div>
        </>
    }
}
