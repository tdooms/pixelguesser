use api::Player;
use pbs::ColumnSize;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub session_id: u64,
    pub players: HashMap<u64, Player>,
}

pub struct Scores {
    props: Props,
}

impl Component for Scores {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut players: Vec<_> = self.props.players.iter().map(|x| x.1).collect();
        players.sort_by_key(|player| std::cmp::Reverse(player.score));

        let view_player = |player: &&Player| {
            html! {
                <pbs::Box extra="is-flex is-flex-direction-row is-justify-content-space-between mx-5">
                    <p class="is-size-5"> {&player.name} </p>
                    <strong class="is-size-5"> {&player.score} </strong>
                </pbs::Box>
            }
        };

        let view_winner = |player: &&Player| {
            html! {
                <pbs::Box extra="is-flex is-flex-direction-row is-justify-content-space-between">
                    <p class="is-size-3"> {&player.name} </p>
                    <strong class="is-size-3"> {&player.score} </strong>
                </pbs::Box>
            }
        };

        html! {
            <pbs::Columns centered=true>
                <pbs::Column size=ColumnSize::IsHalf>
                    { for players.first().map(view_winner) }
                    { for players.iter().skip(1).map(view_player) }
               </pbs::Column>
            </pbs::Columns>
        }
    }
}
