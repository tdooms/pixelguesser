use yew::prelude::*;

use cobul::*;
use sessions::{Player, Session};
use std::rc::Rc;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub session: Rc<Session>,
}

#[function_component(Ranking)]
pub fn ranking(props: &Props) -> Html {
    let mut sorted: Vec<_> = props.session.players.iter().collect();
    sorted.sort_by_key(|(_, player)| std::cmp::Reverse(player.score));

    let view_player = |player: &(&String, &Player)| {
        html! {
            <Box class="is-flex is-flex-direction-row is-justify-content-space-between mx-5">
                <p class="is-size-5"> {player.0.clone()} </p>
                <strong class="is-size-5"> {player.1.score.clone()} </strong>
            </Box>
        }
    };

    let view_winner = |player: &(&String, &Player)| {
        html! {
            <Box class="is-flex is-flex-direction-row is-justify-content-space-between">
                <p class="is-size-3"> {player.0.clone()} </p>
                <strong class="is-size-3"> {player.1.score.clone()} </strong>
            </Box>
        }
    };

    html! {
        <Section class="is-fullheight">
            <div class="columns is-centered is-desktop" style="height:100vh">
                <div class="column is-half">
                    { for sorted.first().map(view_winner) }
                    { for sorted.iter().skip(1).map(view_player) }
                </div>
            </div>
        </Section>
    }
}
