use yew::prelude::*;

use cobul::props::ColumnSize;
use cobul::*;
use sessions::Player;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub players: Vec<Player>,
}

#[function_component(Ranking)]
pub fn ranking(props: &Props) -> Html {
    let mut sorted = props.players.clone();
    sorted.sort_by_key(|player| std::cmp::Reverse(player.score));

    let view_player = |player: &Player| {
        html! {
            <Box extra="is-flex is-flex-direction-row is-justify-content-space-between mx-5">
                <p class="is-size-5"> {player.name.clone()} </p>
                <strong class="is-size-5"> {player.score.clone()} </strong>
            </Box>
        }
    };

    let view_winner = |player: &Player| {
        html! {
            <Box extra="is-flex is-flex-direction-row is-justify-content-space-between">
                <p class="is-size-3"> {player.name.clone()} </p>
                <strong class="is-size-3"> {player.score.clone()} </strong>
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
