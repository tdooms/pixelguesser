use yew::prelude::*;

use cobul::props::ColumnSize;
use cobul::*;
use sessions::Player;
use std::collections::HashMap;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub players: HashMap<String, Player>,
}

#[function_component(Ranking)]
pub fn ranking(props: &Props) -> Html {
    let mut sorted: Vec<_> = props.players.iter().collect();
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
        <Section>
            <Container>
                <Columns centered=true>
                    <Column size={ColumnSize::IsHalf}>
                        { for sorted.first().map(view_winner) }
                        { for sorted.iter().skip(1).map(view_player) }
                   </Column>
                </Columns>
            </Container>
        </Section>
    }
}
